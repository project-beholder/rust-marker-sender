use std::{collections::HashMap};

use opencv::{
    prelude::*,
    core,
    videoio,
    highgui,
    objdetect,
    types::{VectorOfVectorOfPoint2f, VectorOfi32},
};

#[derive(Debug)]
pub struct MarkerInfo {
    id: i32,
    x: f32,
    y: f32,
}

impl MarkerInfo {
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
}

pub struct MarkerDetector {
    cam: videoio::VideoCapture,
    frame: Mat,
    detector: objdetect::ArucoDetector,
    marker_array: Vec<Vec<f32>>,
}

pub fn create_marker_detector() -> MarkerDetector {
    // Set up ArUco
    let dict = objdetect::get_predefined_dictionary(objdetect::PredefinedDictionaryType::DICT_4X4_100).unwrap();
    let param = objdetect::DetectorParameters::default().unwrap();
    let refine = objdetect::RefineParameters::new(10.0, 3.0, true).unwrap();

    let mut detector = MarkerDetector { 
        cam: videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(), 
        frame: Mat::default(), 
        detector: objdetect::ArucoDetector::new(&dict, &param, refine).unwrap(),
        marker_array: Vec::new()
    };
    if (!detector.cam.open(0, videoio::CAP_ANY).unwrap()) {
        panic!("Unable to open camera");
    }

    detector.initialize_vector();

    detector
}

impl MarkerDetector {
    pub fn initialize_vector(&mut self) {
        for i in 0..8 {
            self.marker_array.push(vec![0.0, 0.0]);
        }
    }

    pub fn observation_loop(&mut self) {
        self.cam.read(&mut self.frame).unwrap();

        // Detect markers
        let mut corners: VectorOfVectorOfPoint2f = VectorOfVectorOfPoint2f::new();
        let mut rejected: VectorOfVectorOfPoint2f = VectorOfVectorOfPoint2f::new();
        let mut ids: VectorOfi32 = VectorOfi32::new();

        self.detector.detect_markers(&self.frame, &mut corners, &mut ids, &mut rejected).unwrap();

        if ids.len() > 0 {
            let color: core::Scalar = core::Scalar::new(0.0, 0.0, 255.0, 0.0);
            objdetect::draw_detected_markers(&mut self.frame, &corners, &ids, color).unwrap();
        }

        for i in (0..ids.len()) {
            let id: usize = ids.get(i).unwrap().try_into().unwrap();
            if (id < 8) {
                let x1 = corners.get(i).unwrap().get(0).unwrap().x;
                let y1 = corners.get(i).unwrap().get(0).unwrap().y;
                let x2 = corners.get(i).unwrap().get(2).unwrap().x;
                let y2 = corners.get(i).unwrap().get(2).unwrap().y;
    
                let x = ((x1.floor() + x2.floor()) / 2.0).floor();
                let y = ((y1.floor() + y2.floor()) / 2.0).floor();

                self.marker_array[id][0] = x;
                self.marker_array[id][1] = y;
            }
            // Uncomment line below to show debug camera window
            highgui::imshow("beholder", &self.frame).unwrap();
        }

    } 

    pub fn print_markers(&self) -> String {
        // Print for debugging
        println!("{:?}", self.marker_array);

        let marker_str: String = self.marker_array
            .iter()
            .map( |point| format!("{} {} ", point[0].to_string(), point[1].to_string()))
            .collect();

        marker_str
    }
}