use libohlc::ohlc_maker::{OHLCMaker};
use libohlc::tools::datas::{OHLCData};
use libohlc::tools::tick_generator::{read_lines};

#[test]
fn test_ohlc_make_data_a() {
    // use dataset a to test
    let maker = OHLCMaker::new();
    let tick_path = "../data/dataset-a.txt";
    let ohlc_path = "../data/predictions.txt";
    let reference_ohlc_path = "../data/ohlc-5m-a.txt";
    let window = 60 * 5 * 1000;
    maker.make(tick_path, window, ohlc_path);

    let mut predictions: Vec<OHLCData> = Vec::with_capacity(100);
    if let Ok(lines) = read_lines(&ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                predictions.push(ohlc);
            }      
        }   
    }

    let mut references: Vec<OHLCData> = Vec::with_capacity(100);
    if let Ok(lines) = read_lines(&reference_ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                references.push(ohlc);
            }      
        }   
    }
    let ref_len = references.len();
    let pre_len = predictions.len();
    for (i, reference) in references.into_iter().enumerate() {
        assert_eq!(predictions[i].open.parse::<f64>().unwrap(), reference.open.parse::<f64>().unwrap());
        assert_eq!(predictions[i].high.parse::<f64>().unwrap(), reference.high.parse::<f64>().unwrap());
        assert_eq!(predictions[i].low.parse::<f64>().unwrap(), reference.low.parse::<f64>().unwrap());
        assert_eq!(predictions[i].close.parse::<f64>().unwrap(), reference.close.parse::<f64>().unwrap());
    }
    assert_eq!(ref_len, pre_len);
}

#[test]
fn test_ohlc_make_mock_data() {
    // use my mock data to test
    let maker = OHLCMaker::new();
    let tick_path = "tests/mock_data/dataset.txt";
    let ohlc_path = "tests/mock_data/predictions.txt";
    let reference_ohlc_path = "tests/mock_data/ohlc-50ms.txt";
    let window = 50;  // 50 ms
    maker.make(tick_path, window, ohlc_path);

    let mut predictions: Vec<OHLCData> = Vec::with_capacity(1024);
    if let Ok(lines) = read_lines(&ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                predictions.push(ohlc);
            }      
        }
    }

    let mut references: Vec<OHLCData> = Vec::with_capacity(1024);
    if let Ok(lines) = read_lines(&reference_ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                references.push(ohlc);
            }
        }   
    }

    let ref_len = references.len();
    let pre_len = predictions.len();
    for (i, reference) in references.into_iter().enumerate() {
        assert_eq!(predictions[i].open.parse::<f64>().unwrap(), reference.open.parse::<f64>().unwrap());
        assert_eq!(predictions[i].high.parse::<f64>().unwrap(), reference.high.parse::<f64>().unwrap());
        assert_eq!(predictions[i].low.parse::<f64>().unwrap(), reference.low.parse::<f64>().unwrap());
        assert_eq!(predictions[i].close.parse::<f64>().unwrap(), reference.close.parse::<f64>().unwrap());
    }
    assert_eq!(ref_len, pre_len);
}


#[test]
fn test_ohlc_make_data_a_parallel() {
    // use dataset a to test batch solution
    let maker = OHLCMaker::new();
    let tick_path = "../data/dataset-a.txt";
    let ohlc_path = "../data/predictions.txt";
    let reference_ohlc_path = "../data/ohlc-5m-a.txt";
    let window = 60 * 5 * 1000;
    maker.parallel_make(tick_path, window, ohlc_path);
    
    let mut predictions: Vec<OHLCData> = Vec::with_capacity(100);
    if let Ok(lines) = read_lines(&ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                predictions.push(ohlc);
            }      
        }   
    }

    let mut references: Vec<OHLCData> = Vec::with_capacity(100);
    if let Ok(lines) = read_lines(&reference_ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                references.push(ohlc);
            }      
        }   
    }
    let ref_len = references.len();
    let pre_len = predictions.len();
    for (i, reference) in references.into_iter().enumerate() {
        assert_eq!(predictions[i].open.parse::<f64>().unwrap(), reference.open.parse::<f64>().unwrap());
        assert_eq!(predictions[i].high.parse::<f64>().unwrap(), reference.high.parse::<f64>().unwrap());
        assert_eq!(predictions[i].low.parse::<f64>().unwrap(), reference.low.parse::<f64>().unwrap());
        assert_eq!(predictions[i].close.parse::<f64>().unwrap(), reference.close.parse::<f64>().unwrap());
    }
    assert_eq!(ref_len, pre_len);
}


#[test]
fn test_ohlc_make_mock_data_parallel() {
    // use my mock data to test batch solution
    let maker = OHLCMaker::new();
    let tick_path = "tests/mock_data/dataset.txt";
    let ohlc_path = "tests/mock_data/predictions.txt";
    let reference_ohlc_path = "tests/mock_data/ohlc-50ms.txt";
    let window = 50 ;  // 50ms
    maker.parallel_make(tick_path, window, ohlc_path);

    let mut predictions: Vec<OHLCData> = Vec::with_capacity(1024);
    if let Ok(lines) = read_lines(&ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                predictions.push(ohlc);
            }      
        }
    }

    let mut references: Vec<OHLCData> = Vec::with_capacity(1024);
    if let Ok(lines) = read_lines(&reference_ohlc_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ohlc: OHLCData = serde_json::from_str(ip.as_str()).expect("JSON was not well-formatted");
                references.push(ohlc);
            }
        }   
    }

    let ref_len = references.len();
    let pre_len = predictions.len();
    for (i, reference) in references.into_iter().enumerate() {
        assert_eq!(predictions[i].open.parse::<f64>().unwrap(), reference.open.parse::<f64>().unwrap());
        assert_eq!(predictions[i].high.parse::<f64>().unwrap(), reference.high.parse::<f64>().unwrap());
        assert_eq!(predictions[i].low.parse::<f64>().unwrap(), reference.low.parse::<f64>().unwrap());
        assert_eq!(predictions[i].close.parse::<f64>().unwrap(), reference.close.parse::<f64>().unwrap());
    }
    assert_eq!(ref_len, pre_len);
}