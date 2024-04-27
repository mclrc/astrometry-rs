/// FITS Bintable Reader
use std::{collections::HashMap, error::Error, rc::Rc};

use fitrs::{Fits, FitsData, FitsDataArray, Hdu, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Number, Value};

fn datatype_size(format: &str) -> Result<usize, String> {
    let count = format[..1].parse::<usize>().unwrap();
    let datatype = format.chars().nth(1).unwrap();

    if datatype == 'X' {
        Ok((count as f32 / 8.0).ceil() as usize)
    } else {
        Ok(match datatype {
            'L' => 1,  // Logical (boolean)
            'B' => 1,  // 8-bit byte
            'I' => 2,  // 16-bit integer
            'J' => 4,  // 32-bit integer
            'K' => 8,  // 64-bit integer
            'A' => 1,  // Character
            'E' => 4,  // Single-precision floating point (32-bit float)
            'D' => 8,  // Double-precision floating point (64-bit float)
            'C' => 8,  // Complex floating point (2 x 32-bit)
            'M' => 16, // Double complex floating point (2 x 64-bit)
            'X' => Err("Bit (X) size cannot be given in bytes")?,
            _ => Err(format!(
                "Can't calculate size: unknown column type: {}",
                datatype
            ))?,
        })
    }
}

fn parse_bytes(dtype: char, bytes: &[u8]) -> Result<Value, Box<dyn Error>> {
    Ok(match dtype {
        'L' => Value::Bool(bytes[0] != 0),
        'X' => Value::Number(u8::from_be_bytes(bytes.try_into()?).into()),
        'B' => Value::Number(u8::from_be_bytes(bytes.try_into()?).into()),
        'I' => Value::Number(i16::from_be_bytes(bytes.try_into()?).into()),
        'J' => Value::Number(i32::from_be_bytes(bytes.try_into()?).into()),
        'K' => Value::Number(i64::from_be_bytes(bytes.try_into()?).into()),
        'E' => Value::Number(
            Number::from_f64(f32::from_be_bytes(bytes.try_into()?) as f64)
                .ok_or(format!("Invalid f32: {:?}", bytes))?,
        ),
        'D' => Value::Number(
            Number::from_f64(f64::from_be_bytes(bytes.try_into()?))
                .ok_or(format!("Invalid f64: {:?}", bytes))?,
        ),
        'A' => Value::String(String::from_utf8(bytes.to_vec())?),
        'C' => json!({
            "real": f32::from_be_bytes(bytes[..4].try_into()?),
            "imaginary": f32::from_be_bytes(bytes[4..].try_into()?),
        }),
        'M' => json!({
            "real": f64::from_be_bytes(bytes[..8].try_into()?),
            "imaginary": f64::from_be_bytes(bytes[8..].try_into()?),
        }),
        _ => Err(format!("Can't parse value: unknown column type: {}", dtype))?,
    })
}

fn read_to_string(hdu: &Hdu, key: &str) -> Result<String, String> {
    match hdu.value(key).ok_or(format!("Value not found: {}", key))? {
        HeaderValue::CharacterString(s) => Ok(s.clone()),
        v => Err(format!("{} Not a string: {:?}", key, v)),
    }
}

pub struct ColumnIter<'a> {
    row_size: usize,
    row: usize,
    column: [usize; 2],
    data: &'a [u8],
}

impl<'a> ColumnIter<'a> {
    pub fn new(data: &'a [u8], row_size: usize, column: [usize; 2]) -> ColumnIter<'a> {
        ColumnIter {
            data,
            row_size,
            row: 0,
            column,
        }
    }
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.data.len() / self.row_size {
            return None;
        }
        let offset = self.row * self.row_size + self.column[0];
        let end = offset + self.column[1];
        self.row += 1;
        Some(&self.data[offset..end])
    }
}

pub struct FitsTableData {
    shape: Vec<usize>,
    data: Vec<u8>,
}

pub struct FitsColumn {
    index: usize,
    name: String,
    format: String,
    unit: Option<String>,
    datatype: char,
    count: usize,
    size: usize,
    offset: usize,
    data: Rc<FitsTableData>,
}

impl FitsColumn {
    pub fn value(&self, row: usize) -> Result<Value, Box<dyn Error>> {
        let bytes =
            &self.data.data[row * self.data.shape[0]..][self.offset..self.offset + self.size];

        if self.count == 1 || self.datatype == 'X' {
            parse_bytes(self.datatype, bytes)
        } else {
            let array = Value::Array(
                bytes
                    .chunks(datatype_size(&self.format)?)
                    .map(|chunk| parse_bytes(self.datatype, chunk))
                    .collect::<Result<Vec<_>, _>>()?,
            );

            Ok(array)
        }
    }

    pub fn iter(&self) -> ColumnIter {
        ColumnIter::new(
            &self.data.data,
            self.data.shape[0],
            [self.offset, self.size],
        )
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}

pub struct FitsTable {
    hdu: Hdu,
    data: Rc<FitsTableData>,
    columns: HashMap<String, FitsColumn>,
}

impl FitsTable {
    pub fn open(path: &str, hdu_index: usize) -> Result<Self, Box<dyn Error>> {
        let hdu = Fits::open(path)?
            .get(hdu_index)
            .ok_or("HDU does not exist")?;

        let data = match hdu.read_data() {
            FitsData::Bytes(FitsDataArray { shape, data }) => FitsTableData { shape, data },
            _ => Err("Table data not in bytes")?,
        };

        let data = Rc::new(data);

        let table = FitsTable {
            data: Rc::clone(&data),
            columns: Self::get_columns(&hdu, Rc::clone(&data))?,
            hdu,
        };

        Ok(table)
    }

    fn get_columns(
        hdu: &Hdu,
        data: Rc<FitsTableData>,
    ) -> Result<HashMap<String, FitsColumn>, Box<dyn Error>> {
        let nfields = match hdu.value("TFIELDS").unwrap() {
            HeaderValue::IntegerNumber(n) => *n,
            v => panic!("TFIELDS Not an integer: {:?}", v),
        };

        let mut columns = HashMap::new();
        let mut offset = 0;

        for i in 1..=nfields {
            let format = read_to_string(hdu, &format!("TFORM{}", i))?;
            let name = read_to_string(hdu, &format!("TTYPE{}", i))?;
            let unit = read_to_string(hdu, &format!("TUNIT{}", i)).ok();

            let count = format[..1].parse::<usize>().unwrap();
            let datatype = format.chars().nth(1).unwrap();

            let column_size = datatype_size(&format)?;

            columns.insert(
                name.clone(),
                FitsColumn {
                    index: columns.len(),
                    name,
                    format,
                    datatype,
                    count,
                    unit,
                    size: column_size,
                    data: Rc::clone(&data),
                    offset,
                },
            );

            offset += column_size;
        }

        Ok(columns)
    }

    pub fn deserialize_row<T: DeserializeOwned>(&self, row: usize) -> Result<T, Box<dyn Error>> {
        let mut values = Map::new();

        for (column_name, column) in self.columns.iter() {
            values.insert(column_name.clone(), column.value(row)?);
        }

        Ok(serde_json::from_value(Value::Object(values))?)
    }

    pub fn iter<T: DeserializeOwned>(
        &self,
    ) -> impl Iterator<Item = Result<T, Box<dyn Error>>> + '_ {
        (0..self.data.shape[1]).map(|row| self.deserialize_row(row))
    }

    pub fn hdu(&self) -> &Hdu {
        &self.hdu
    }

    pub fn columns(&self) -> &HashMap<String, FitsColumn> {
        &self.columns
    }

    pub fn len(&self) -> usize {
        self.data.shape[1]
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
