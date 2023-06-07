extern crate serde_big_array;

use serde::{Serialize, Deserialize};

use crate::usb::get_graph;

#[derive(Serialize, Deserialize)]
pub struct Graph {
    x: Vec<f32>,
    y: Vec<f32>,
}

#[tauri::command]
pub fn grafico() -> Graph {
    let mut ret = Vec::new();
    let str = get_graph();
    for i in str {
        ret.push(i.replace(|c: char| !c.is_digit(10) && c!='.' && c!='-', "").parse().unwrap());
    }
    let eje = (0..ret.len()).map(|x| x as f32 * 0.1).collect();
    println!("{:?}", eje);
    Graph{ 
        x: eje,
        y: ret,

    }
}
