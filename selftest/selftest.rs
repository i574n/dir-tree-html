#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
enum US0 {
    US0_Ok(Vec<u8>),
    US0_Error(std::io::Error),
}
#[derive(Clone)]
enum US1 {
    US1_Ok(std::string::String),
    US1_Error(std::string::FromUtf8Error),
}
fn go_0(mut v0: i32, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 <= 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: u64 = v1 & 15u64;
            let v5: i32 = (v4 as i32);
            let v6: i32 = v0.wrapping_sub(1i32);
            let v7: u64 = v1.wrapping_shr((4i32) as u32);
            let v8: Rc<str> = string_slice(&{ thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789abcdef"); } LIT.with(|lit| lit.clone()) }, v5 as i64, v5 as i64);
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
            (v0, v1, v2) = (v6, v7, v9.clone());
            continue;
        }
    }
}
fn times_2(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let v2: bool = v1 <= 0i64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let v4: i64 = v1.wrapping_sub(1i64);
        let v5: Rc<str> = times_2(v0.clone(), v4);
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
        v6.clone()
    }
}
fn last_sep_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 == v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'/';
            let v9: i32 = if v6 {
                v1
            } else {
                let v7: bool = v5 == b'\\';
                if v7 {
                    v1
                } else {
                    v2
                }
            };
            let v10: i32 = v1.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v10, v9);
            continue;
        }
    }
}
fn lay_1(mut v0: Rc<str>, mut v1: i64) -> i32 {
    loop {
        let v2: bool = v1 < 0i64;
        if v2 {
            return 0i32;
        } else {
            let v4: i32 = i32::from(std::fs::create_dir_all(v0.as_ref()).is_err());
            let v5: bool = v1 == 0i64;
            let v35: Rc<str> = if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            } else {
                let v7: bool = v1 == 1i64;
                if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                } else {
                    let v9: bool = v1 == 2i64;
                    if v9 {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
                        v10.clone()
                    } else {
                        let v11: bool = v1 == 3i64;
                        if v11 {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        } else {
                            let v13: bool = v1 == 4i64;
                            if v13 {
                                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                                v14.clone()
                            } else {
                                let v15: bool = v1 == 5i64;
                                if v15 {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5"); } LIT.with(|lit| lit.clone()) };
                                    v16.clone()
                                } else {
                                    let v17: bool = v1 == 6i64;
                                    if v17 {
                                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
                                        v18.clone()
                                    } else {
                                        let v19: bool = v1 == 7i64;
                                        if v19 {
                                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("7"); } LIT.with(|lit| lit.clone()) };
                                            v20.clone()
                                        } else {
                                            let v21: bool = v1 == 8i64;
                                            if v21 {
                                                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("8"); } LIT.with(|lit| lit.clone()) };
                                                v22.clone()
                                            } else {
                                                let v23: bool = v1 == 9i64;
                                                if v23 {
                                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("9"); } LIT.with(|lit| lit.clone()) };
                                                    v24.clone()
                                                } else {
                                                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                                                    v25.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v37.as_ref()).display().to_string());
            let v39: i64 = v1.wrapping_add(1i64);
            let v40: Rc<str> = times_2(v35.clone(), v39);
            let v41: i32 = 0i32;
            let v42: i32 = -1i32;
            let v43: i32 = last_sep_3(v38.clone(), v41, v42);
            let v44: bool = v43 == -1i32;
            let v55: Rc<str> = if v44 {
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v45.clone()
            } else {
                let v46: i32 = v43.wrapping_sub(1i32);
                let v47: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v46 as i64);
                let v48: i32 = (v47.clone().len() as i32);
                let v49: bool = v48 == 2i32;
                let v52: bool = if v49 {
                    let v50: u8 = v47.clone().as_bytes()[1i32 as usize];
                    let v51: bool = v50 == b':';
                    v51
                } else {
                    false
                };
                if v52 {
                    let v53: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v43 as i64);
                    v53.clone()
                } else {
                    v47.clone()
                }
            };
            let v56: i32 = (v55.clone().len() as i32);
            let v57: bool = v56 == 0i32;
            let v60: i32 = if v57 {
                0i32
            } else {
                let v59: i32 = i32::from(std::fs::create_dir_all(v55.as_ref()).is_err());
                v59
            };
            let v61: bool = v60 == 0i32;
            let v64: i32 = if v61 {
                let v63: i32 = i32::from(std::fs::write(v38.as_ref(), v40.as_ref().as_bytes()).is_err());
                v63
            } else {
                v60
            };
            let v66: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v35.as_ref()).display().to_string());
            let v67: i64 = v1.wrapping_sub(1i64);
            (v0, v1) = (v66.clone(), v67);
            continue;
        }
    }
}
fn ok_c1() -> Rc<dyn Fn(Vec<u8>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<u8>) -> US0> = Rc::new(move |mut v0: Vec<u8>| -> US0 {
        US0::US0_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4() -> Rc<dyn Fn(Vec<u8>) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US0> = Rc::new(move |mut v0: std::io::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5() -> Rc<dyn Fn(std::io::Error) -> US0> {
    error_c2()
}
fn method6(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn ok_c3() -> Rc<dyn Fn(std::string::String) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::String) -> US1> = Rc::new(move |mut v0: std::string::String| -> US1 {
        US1::US1_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7() -> Rc<dyn Fn(std::string::String) -> US1> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::FromUtf8Error) -> US1> = Rc::new(move |mut v0: std::string::FromUtf8Error| -> US1 {
        US1::US1_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US1> {
    error_c4()
}
fn closure0() -> Rc<dyn Fn(i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> i32> = Rc::new(move |mut v0: i32| -> i32 {
        let v2: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let mut v3: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
        let v5: bool = true; // 1;
        let mut v3 = v3;
        let v7: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        std::hash::Hasher::write_u128(&mut v3, v7);
        let v8: u64 = std::hash::Hasher::finish(&v3);
        std::hash::Hasher::write_u64(&mut v3, v8);
        let v9: u64 = std::hash::Hasher::finish(&v3);
        let v10: u64 = v8 & 4294967295u64;
        let v11: i32 = 8i32;
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = go_0(v11, v10, v12.clone());
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
        let v16: u64 = v8.wrapping_shr((32i32) as u32);
        let v17: u64 = v16 & 65535u64;
        let v18: i32 = 4i32;
        let v19: Rc<str> = go_0(v18, v17, v12.clone());
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v19));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-4"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let v23: u64 = v8.wrapping_shr((48i32) as u32);
        let v24: u64 = v23 & 4095u64;
        let v25: i32 = 3i32;
        let v26: Rc<str> = go_0(v25, v24, v12.clone());
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v26)), v14));
        let v29: u64 = v9 & 16383u64;
        let v30: u64 = v29 | 32768u64;
        let v31: i32 = 4i32;
        let v32: Rc<str> = go_0(v31, v30, v12.clone());
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v32)), v14));
        let v35: u64 = v9.wrapping_shr((16i32) as u32);
        let v36: u64 = v35 & 281474976710655u64;
        let v37: i32 = 12i32;
        let v38: Rc<str> = go_0(v37, v36, v12.clone());
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v38));
        let v40: Rc<str> = Rc::<str>::from(v39.to_lowercase());
        let v41: Rc<str> = v40 ;
        let v43: Rc<str> = Rc::<str>::from(std::path::Path::new(v2.as_ref()).join(v41.as_ref()).display().to_string());
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_.root"); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = Rc::<str>::from(std::path::Path::new(v43.as_ref()).join(v45.as_ref()).display().to_string());
        let v47: i64 = 3i64;
        let v48: i32 = lay_1(v46.clone(), v47);
        let v49: Rc<str> = dtl_tree::dtl_tree_html(&*v43);
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v49));
        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v51, Rc::<str>::from(format!("{}{}", v53, v54))));
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div><details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v58, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v12)), v54))));
        let v62: bool = &*v56 == &*v61;
        let v91: i32 = if v62 {
            0i32
        } else {
            let v64: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-got.html"); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = Rc::<str>::from(std::path::Path::new(v64.as_ref()).join(v66.as_ref()).display().to_string());
            let v68: i32 = 0i32;
            let v69: i32 = -1i32;
            let v70: i32 = last_sep_3(v67.clone(), v68, v69);
            let v71: bool = v70 == -1i32;
            let v81: Rc<str> = if v71 {
                v12.clone()
            } else {
                let v72: i32 = v70.wrapping_sub(1i32);
                let v73: Rc<str> = string_slice(&v67.clone(), 0i32 as i64, v72 as i64);
                let v74: i32 = (v73.clone().len() as i32);
                let v75: bool = v74 == 2i32;
                let v78: bool = if v75 {
                    let v76: u8 = v73.clone().as_bytes()[1i32 as usize];
                    let v77: bool = v76 == b':';
                    v77
                } else {
                    false
                };
                if v78 {
                    let v79: Rc<str> = string_slice(&v67.clone(), 0i32 as i64, v70 as i64);
                    v79.clone()
                } else {
                    v73.clone()
                }
            };
            let v82: i32 = (v81.clone().len() as i32);
            let v83: bool = v82 == 0i32;
            let v86: i32 = if v83 {
                0i32
            } else {
                let v85: i32 = i32::from(std::fs::create_dir_all(v81.as_ref()).is_err());
                v85
            };
            let v87: bool = v86 == 0i32;
            let v90: i32 = if v87 {
                let v89: i32 = i32::from(std::fs::write(v67.as_ref(), v56.as_ref().as_bytes()).is_err());
                v89
            } else {
                v86
            };
            1i32
        };
        let v92: i64 = dtl_tree::dtl_tree_bytes(&*v43);
        let v93: bool = v92 == 10i64;
        let v94: i32 = if v93 {
            0i32
        } else {
            3i32
        };
        let v96: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let mut v97: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
        let v99: bool = true; // 1;
        let mut v97 = v97;
        let v101: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        std::hash::Hasher::write_u128(&mut v97, v101);
        let v102: u64 = std::hash::Hasher::finish(&v97);
        std::hash::Hasher::write_u64(&mut v97, v102);
        let v103: u64 = std::hash::Hasher::finish(&v97);
        let v104: u64 = v102 & 4294967295u64;
        let v105: i32 = 8i32;
        let v106: Rc<str> = go_0(v105, v104, v12.clone());
        let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v14));
        let v108: u64 = v102.wrapping_shr((32i32) as u32);
        let v109: u64 = v108 & 65535u64;
        let v110: i32 = 4i32;
        let v111: Rc<str> = go_0(v110, v109, v12.clone());
        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v107, v111)), v21));
        let v114: u64 = v102.wrapping_shr((48i32) as u32);
        let v115: u64 = v114 & 4095u64;
        let v116: i32 = 3i32;
        let v117: Rc<str> = go_0(v116, v115, v12.clone());
        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v113, v117)), v14));
        let v120: u64 = v103 & 16383u64;
        let v121: u64 = v120 | 32768u64;
        let v122: i32 = 4i32;
        let v123: Rc<str> = go_0(v122, v121, v12.clone());
        let v125: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v119, v123)), v14));
        let v126: u64 = v103.wrapping_shr((16i32) as u32);
        let v127: u64 = v126 & 281474976710655u64;
        let v128: i32 = 12i32;
        let v129: Rc<str> = go_0(v128, v127, v12.clone());
        let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v129));
        let v131: Rc<str> = Rc::<str>::from(v130.to_lowercase());
        let v132: Rc<str> = v131 ;
        let v134: Rc<str> = Rc::<str>::from(std::path::Path::new(v96.as_ref()).join(v132.as_ref()).display().to_string());
        let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let v137: Rc<str> = Rc::<str>::from(std::path::Path::new(v134.as_ref()).join(v136.as_ref()).display().to_string());
        let v139: i32 = i32::from(std::fs::create_dir_all(v137.as_ref()).is_err());
        let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.txt"); } LIT.with(|lit| lit.clone()) };
        let v142: Rc<str> = Rc::<str>::from(std::path::Path::new(v134.as_ref()).join(v141.as_ref()).display().to_string());
        let v143: i32 = 0i32;
        let v144: i32 = -1i32;
        let v145: i32 = last_sep_3(v142.clone(), v143, v144);
        let v146: bool = v145 == -1i32;
        let v156: Rc<str> = if v146 {
            v12.clone()
        } else {
            let v147: i32 = v145.wrapping_sub(1i32);
            let v148: Rc<str> = string_slice(&v142.clone(), 0i32 as i64, v147 as i64);
            let v149: i32 = (v148.clone().len() as i32);
            let v150: bool = v149 == 2i32;
            let v153: bool = if v150 {
                let v151: u8 = v148.clone().as_bytes()[1i32 as usize];
                let v152: bool = v151 == b':';
                v152
            } else {
                false
            };
            if v153 {
                let v154: Rc<str> = string_slice(&v142.clone(), 0i32 as i64, v145 as i64);
                v154.clone()
            } else {
                v148.clone()
            }
        };
        let v157: i32 = (v156.clone().len() as i32);
        let v158: bool = v157 == 0i32;
        let v161: i32 = if v158 {
            0i32
        } else {
            let v160: i32 = i32::from(std::fs::create_dir_all(v156.as_ref()).is_err());
            v160
        };
        let v162: bool = v161 == 0i32;
        let v166: i32 = if v162 {
            let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x"); } LIT.with(|lit| lit.clone()) };
            let v165: i32 = i32::from(std::fs::write(v142.as_ref(), v164.as_ref().as_bytes()).is_err());
            v165
        } else {
            v161
        };
        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.txt"); } LIT.with(|lit| lit.clone()) };
        let v169: Rc<str> = Rc::<str>::from(std::path::Path::new(v134.as_ref()).join(v168.as_ref()).display().to_string());
        let v170: i32 = 0i32;
        let v171: i32 = -1i32;
        let v172: i32 = last_sep_3(v169.clone(), v170, v171);
        let v173: bool = v172 == -1i32;
        let v183: Rc<str> = if v173 {
            v12.clone()
        } else {
            let v174: i32 = v172.wrapping_sub(1i32);
            let v175: Rc<str> = string_slice(&v169.clone(), 0i32 as i64, v174 as i64);
            let v176: i32 = (v175.clone().len() as i32);
            let v177: bool = v176 == 2i32;
            let v180: bool = if v177 {
                let v178: u8 = v175.clone().as_bytes()[1i32 as usize];
                let v179: bool = v178 == b':';
                v179
            } else {
                false
            };
            if v180 {
                let v181: Rc<str> = string_slice(&v169.clone(), 0i32 as i64, v172 as i64);
                v181.clone()
            } else {
                v175.clone()
            }
        };
        let v184: i32 = (v183.clone().len() as i32);
        let v185: bool = v184 == 0i32;
        let v188: i32 = if v185 {
            0i32
        } else {
            let v187: i32 = i32::from(std::fs::create_dir_all(v183.as_ref()).is_err());
            v187
        };
        let v189: bool = v188 == 0i32;
        let v193: i32 = if v189 {
            let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yy"); } LIT.with(|lit| lit.clone()) };
            let v192: i32 = i32::from(std::fs::write(v169.as_ref(), v191.as_ref().as_bytes()).is_err());
            v192
        } else {
            v188
        };
        let v195: Rc<str> = Rc::<str>::from(std::path::Path::new(v134.as_ref()).join(v136.as_ref()).display().to_string());
        let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
        let v198: Rc<str> = Rc::<str>::from(std::path::Path::new(v195.as_ref()).join(v197.as_ref()).display().to_string());
        let v199: i32 = 0i32;
        let v200: i32 = -1i32;
        let v201: i32 = last_sep_3(v198.clone(), v199, v200);
        let v202: bool = v201 == -1i32;
        let v212: Rc<str> = if v202 {
            v12.clone()
        } else {
            let v203: i32 = v201.wrapping_sub(1i32);
            let v204: Rc<str> = string_slice(&v198.clone(), 0i32 as i64, v203 as i64);
            let v205: i32 = (v204.clone().len() as i32);
            let v206: bool = v205 == 2i32;
            let v209: bool = if v206 {
                let v207: u8 = v204.clone().as_bytes()[1i32 as usize];
                let v208: bool = v207 == b':';
                v208
            } else {
                false
            };
            if v209 {
                let v210: Rc<str> = string_slice(&v198.clone(), 0i32 as i64, v201 as i64);
                v210.clone()
            } else {
                v204.clone()
            }
        };
        let v213: i32 = (v212.clone().len() as i32);
        let v214: bool = v213 == 0i32;
        let v217: i32 = if v214 {
            0i32
        } else {
            let v216: i32 = i32::from(std::fs::create_dir_all(v212.as_ref()).is_err());
            v216
        };
        let v218: bool = v217 == 0i32;
        let v222: i32 = if v218 {
            let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("z"); } LIT.with(|lit| lit.clone()) };
            let v221: i32 = i32::from(std::fs::write(v198.as_ref(), v220.as_ref().as_bytes()).is_err());
            v221
        } else {
            v217
        };
        let v223: Rc<str> = dtl_tree::dtl_tree_html(&*v134);
        let v224: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v223));
        let v227: Rc<str> = Rc::<str>::from(format!("{}{}", v224, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v12)), v54))));
        let v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"m\">m</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"m/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"a.txt\">a.txt</a><span> (2.00 B)</span></div><div>&#128196; <a href=\"b.txt\">b.txt</a><span> (1.00 B)</span></div>"); } LIT.with(|lit| lit.clone()) };
        let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div><details open=\"true\"><summary>&#128194; <a href=\"m\">m</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"m/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"a.txt\">a.txt</a><span> (2.00 B)</span></div><div>&#128196; <a href=\"b.txt\">b.txt</a><span> (1.00 B)</span></div>"); } LIT.with(|lit| lit.clone()) };
        let v232: Rc<str> = Rc::<str>::from(format!("{}{}", v229, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v12)), v54))));
        let v233: bool = &*v227 == &*v232;
        let v262: i32 = if v233 {
            0i32
        } else {
            let v235: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-order-got.html"); } LIT.with(|lit| lit.clone()) };
            let v238: Rc<str> = Rc::<str>::from(std::path::Path::new(v235.as_ref()).join(v237.as_ref()).display().to_string());
            let v239: i32 = 0i32;
            let v240: i32 = -1i32;
            let v241: i32 = last_sep_3(v238.clone(), v239, v240);
            let v242: bool = v241 == -1i32;
            let v252: Rc<str> = if v242 {
                v12.clone()
            } else {
                let v243: i32 = v241.wrapping_sub(1i32);
                let v244: Rc<str> = string_slice(&v238.clone(), 0i32 as i64, v243 as i64);
                let v245: i32 = (v244.clone().len() as i32);
                let v246: bool = v245 == 2i32;
                let v249: bool = if v246 {
                    let v247: u8 = v244.clone().as_bytes()[1i32 as usize];
                    let v248: bool = v247 == b':';
                    v248
                } else {
                    false
                };
                if v249 {
                    let v250: Rc<str> = string_slice(&v238.clone(), 0i32 as i64, v241 as i64);
                    v250.clone()
                } else {
                    v244.clone()
                }
            };
            let v253: i32 = (v252.clone().len() as i32);
            let v254: bool = v253 == 0i32;
            let v257: i32 = if v254 {
                0i32
            } else {
                let v256: i32 = i32::from(std::fs::create_dir_all(v252.as_ref()).is_err());
                v256
            };
            let v258: bool = v257 == 0i32;
            let v261: i32 = if v258 {
                let v260: i32 = i32::from(std::fs::write(v238.as_ref(), v227.as_ref().as_bytes()).is_err());
                v260
            } else {
                v257
            };
            1i32
        };
        let v264: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let mut v265: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
        let v267: bool = true; // 1;
        let mut v265 = v265;
        let v269: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        std::hash::Hasher::write_u128(&mut v265, v269);
        let v270: u64 = std::hash::Hasher::finish(&v265);
        std::hash::Hasher::write_u64(&mut v265, v270);
        let v271: u64 = std::hash::Hasher::finish(&v265);
        let v272: u64 = v270 & 4294967295u64;
        let v273: i32 = 8i32;
        let v274: Rc<str> = go_0(v273, v272, v12.clone());
        let v275: Rc<str> = Rc::<str>::from(format!("{}{}", v274, v14));
        let v276: u64 = v270.wrapping_shr((32i32) as u32);
        let v277: u64 = v276 & 65535u64;
        let v278: i32 = 4i32;
        let v279: Rc<str> = go_0(v278, v277, v12.clone());
        let v281: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v275, v279)), v21));
        let v282: u64 = v270.wrapping_shr((48i32) as u32);
        let v283: u64 = v282 & 4095u64;
        let v284: i32 = 3i32;
        let v285: Rc<str> = go_0(v284, v283, v12.clone());
        let v287: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v281, v285)), v14));
        let v288: u64 = v271 & 16383u64;
        let v289: u64 = v288 | 32768u64;
        let v290: i32 = 4i32;
        let v291: Rc<str> = go_0(v290, v289, v12.clone());
        let v293: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v287, v291)), v14));
        let v294: u64 = v271.wrapping_shr((16i32) as u32);
        let v295: u64 = v294 & 281474976710655u64;
        let v296: i32 = 12i32;
        let v297: Rc<str> = go_0(v296, v295, v12.clone());
        let v298: Rc<str> = Rc::<str>::from(format!("{}{}", v293, v297));
        let v299: Rc<str> = Rc::<str>::from(v298.to_lowercase());
        let v300: Rc<str> = v299 ;
        let v302: Rc<str> = Rc::<str>::from(std::path::Path::new(v264.as_ref()).join(v300.as_ref()).display().to_string());
        let v304: i32 = i32::from(std::fs::create_dir_all(v302.as_ref()).is_err());
        let v306: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
        let v307: Rc<str> = Rc::<str>::from(std::path::Path::new(v302.as_ref()).join(v306.as_ref()).display().to_string());
        let v308: i32 = 0i32;
        let v309: i32 = -1i32;
        let v310: i32 = last_sep_3(v307.clone(), v308, v309);
        let v311: bool = v310 == -1i32;
        let v321: Rc<str> = if v311 {
            v12.clone()
        } else {
            let v312: i32 = v310.wrapping_sub(1i32);
            let v313: Rc<str> = string_slice(&v307.clone(), 0i32 as i64, v312 as i64);
            let v314: i32 = (v313.clone().len() as i32);
            let v315: bool = v314 == 2i32;
            let v318: bool = if v315 {
                let v316: u8 = v313.clone().as_bytes()[1i32 as usize];
                let v317: bool = v316 == b':';
                v317
            } else {
                false
            };
            if v318 {
                let v319: Rc<str> = string_slice(&v307.clone(), 0i32 as i64, v310 as i64);
                v319.clone()
            } else {
                v313.clone()
            }
        };
        let v322: i32 = (v321.clone().len() as i32);
        let v323: bool = v322 == 0i32;
        let v326: i32 = if v323 {
            0i32
        } else {
            let v325: i32 = i32::from(std::fs::create_dir_all(v321.as_ref()).is_err());
            v325
        };
        let v327: bool = v326 == 0i32;
        let v331: i32 = if v327 {
            let v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# Title\n\nSome `code` and [a link](x.html).\n\n- one\n- two <b>\n\n봉쥬르 & é [깃허브](k.html)\n\nHi! ![logo](logo.png) [![CI](b.svg)](ci.html)\n\n**Bold** and <https://x.org> and \\*star\\*\n\n---\n\n- top\n  - nested\n\n```\nlet x = 1 < 2\n```\n"); } LIT.with(|lit| lit.clone()) };
            let v330: i32 = i32::from(std::fs::write(v307.as_ref(), v329.as_ref().as_bytes()).is_err());
            v330
        } else {
            v326
        };
        let v333: Rc<str> = Rc::<str>::from(std::path::Path::new(v302.as_ref()).join(v306.as_ref()).display().to_string());
        let v335: bool = std::path::Path::new(v333.as_ref()).is_file();
        let v374: Rc<str> = if v335 {
            let v337: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v333);
            let mut v338: Rc<dyn Fn(Vec<u8>) -> US0> = method4();
            let mut v339: Rc<dyn Fn(std::io::Error) -> US0> = method5();
            let v340: US0 = match v337 { Ok(x) => v338(x), Err(e) => v339(e) };
            let v368: Rc<str> = match v340 {
                US0::US0_Error(v341) => {
                    let v341: std::io::Error = v341;
                    v12.clone()
                }
                US0::US0_Ok(v342) => {
                    let v342: Vec<u8> = v342.clone();
                    let v343: Vec<u8> = method6(v342.clone());
                    let v345: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v343);
                    let mut v346: Rc<dyn Fn(std::string::String) -> US1> = method7();
                    let mut v347: Rc<dyn Fn(std::string::FromUtf8Error) -> US1> = method8();
                    let v348: US1 = match v345 { Ok(x) => v346(x), Err(e) => v347(e) };
                    match &v348 {
                        US1::US1_Error(v349) => {
                            let v349: std::string::FromUtf8Error = v349.clone();
                            v12.clone()
                        }
                        US1::US1_Ok(v350) => {
                            let v350: std::string::String = v350.clone();
                            let v352: Rc<str> = Rc::<str>::from(String::as_str(&v350));
                            let v353: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
                            let v354: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v355: Rc<str> = Rc::<str>::from(v352.replace(&*v353, &*v354));
                            let v356: bool = v355.ends_with(&*v354);
                            let v359: bool = if v356 {
                                let v357: i32 = (v355.clone().len() as i32);
                                let v358: bool = v357 == 1i32;
                                v358
                            } else {
                                false
                            };
                            if v359 {
                                v12.clone()
                            } else {
                                if v356 {
                                    let v360: i32 = (v355.clone().len() as i32);
                                    let v361: i32 = v360.wrapping_sub(2i32);
                                    let v362: Rc<str> = string_slice(&v355.clone(), 0i32 as i64, v361 as i64);
                                    v362.clone()
                                } else {
                                    v355.clone()
                                }
                            }
                        }
                    }
                }
            };
            let v369: Rc<str> = dtl_markdown::dtl_markdown(&*v368);
            let v370: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
            let v371: Rc<str> = Rc::<str>::from(format!("{}{}", v370, v369));
            let v372: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
            let v373: Rc<str> = Rc::<str>::from(format!("{}{}", v371, v372));
            v373.clone()
        } else {
            v12.clone()
        };
        let v375: bool = &*v374 == "<section class=\"readme\">\n<h1>Title</h1>\n<p>Some <code>code</code> and <a href=\"x.html\">a link</a>.</p>\n<ul>\n<li>one</li>\n<li>two &lt;b&gt;</li>\n</ul>\n<p>봉쥬르 &amp; é <a href=\"k.html\">깃허브</a></p>\n<p>Hi! <img src=\"logo.png\" alt=\"logo\"> <a href=\"ci.html\"><img src=\"b.svg\" alt=\"CI\"></a></p>\n<p><strong>Bold</strong> and <a href=\"https://x.org\">https://x.org</a> and *star*</p>\n<hr>\n<ul>\n<li>top</li>\n<li>nested</li>\n</ul>\n<pre><code>let x = 1 &lt; 2\n</code></pre>\n</section>\n";
        let v404: i32 = if v375 {
            0i32
        } else {
            let v377: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let v379: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-readme-got.html"); } LIT.with(|lit| lit.clone()) };
            let v380: Rc<str> = Rc::<str>::from(std::path::Path::new(v377.as_ref()).join(v379.as_ref()).display().to_string());
            let v381: i32 = 0i32;
            let v382: i32 = -1i32;
            let v383: i32 = last_sep_3(v380.clone(), v381, v382);
            let v384: bool = v383 == -1i32;
            let v394: Rc<str> = if v384 {
                v12.clone()
            } else {
                let v385: i32 = v383.wrapping_sub(1i32);
                let v386: Rc<str> = string_slice(&v380.clone(), 0i32 as i64, v385 as i64);
                let v387: i32 = (v386.clone().len() as i32);
                let v388: bool = v387 == 2i32;
                let v391: bool = if v388 {
                    let v389: u8 = v386.clone().as_bytes()[1i32 as usize];
                    let v390: bool = v389 == b':';
                    v390
                } else {
                    false
                };
                if v391 {
                    let v392: Rc<str> = string_slice(&v380.clone(), 0i32 as i64, v383 as i64);
                    v392.clone()
                } else {
                    v386.clone()
                }
            };
            let v395: i32 = (v394.clone().len() as i32);
            let v396: bool = v395 == 0i32;
            let v399: i32 = if v396 {
                0i32
            } else {
                let v398: i32 = i32::from(std::fs::create_dir_all(v394.as_ref()).is_err());
                v398
            };
            let v400: bool = v399 == 0i32;
            let v403: i32 = if v400 {
                let v402: i32 = i32::from(std::fs::write(v380.as_ref(), v374.as_ref().as_bytes()).is_err());
                v402
            } else {
                v399
            };
            1i32
        };
        let v405: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>r</p>\n"); } LIT.with(|lit| lit.clone()) };
        let v407: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v405)), v54));
        let v408: bool = &*v407 == "</div>\n<p>r</p>\n</body>\n</html>\n";
        let v437: i32 = if v408 {
            0i32
        } else {
            let v410: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let v412: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-close-got.html"); } LIT.with(|lit| lit.clone()) };
            let v413: Rc<str> = Rc::<str>::from(std::path::Path::new(v410.as_ref()).join(v412.as_ref()).display().to_string());
            let v414: i32 = 0i32;
            let v415: i32 = -1i32;
            let v416: i32 = last_sep_3(v413.clone(), v414, v415);
            let v417: bool = v416 == -1i32;
            let v427: Rc<str> = if v417 {
                v12.clone()
            } else {
                let v418: i32 = v416.wrapping_sub(1i32);
                let v419: Rc<str> = string_slice(&v413.clone(), 0i32 as i64, v418 as i64);
                let v420: i32 = (v419.clone().len() as i32);
                let v421: bool = v420 == 2i32;
                let v424: bool = if v421 {
                    let v422: u8 = v419.clone().as_bytes()[1i32 as usize];
                    let v423: bool = v422 == b':';
                    v423
                } else {
                    false
                };
                if v424 {
                    let v425: Rc<str> = string_slice(&v413.clone(), 0i32 as i64, v416 as i64);
                    v425.clone()
                } else {
                    v419.clone()
                }
            };
            let v428: i32 = (v427.clone().len() as i32);
            let v429: bool = v428 == 0i32;
            let v432: i32 = if v429 {
                0i32
            } else {
                let v431: i32 = i32::from(std::fs::create_dir_all(v427.as_ref()).is_err());
                v431
            };
            let v433: bool = v432 == 0i32;
            let v436: i32 = if v433 {
                let v435: i32 = i32::from(std::fs::write(v413.as_ref(), v407.as_ref().as_bytes()).is_err());
                v435
            } else {
                v432
            };
            1i32
        };
        let v438: Rc<str> = dtl_tree::dtl_size_label(1024i64);
        let v439: bool = &*v438 == "1024.00 B";
        let v449: i32 = if v439 {
            let v440: Rc<str> = dtl_tree::dtl_size_label(1025i64);
            let v441: bool = &*v440 == "1.00 KB";
            if v441 {
                let v442: Rc<str> = dtl_tree::dtl_size_label(1048576i64);
                let v443: bool = &*v442 == "1024.00 KB";
                if v443 {
                    let v444: Rc<str> = dtl_tree::dtl_size_label(1048577i64);
                    let v445: bool = &*v444 == "1.00 MB";
                    if v445 {
                        0i32
                    } else {
                        1i32
                    }
                } else {
                    1i32
                }
            } else {
                1i32
            }
        } else {
            1i32
        };
        let v450: bool = v91 == 0i32;
        if v450 {
            let v451: bool = v94 == 0i32;
            if v451 {
                let v452: bool = v262 == 0i32;
                if v452 {
                    let v453: bool = v404 == 0i32;
                    if v453 {
                        let v454: bool = v437 == 0i32;
                        if v454 {
                            v449
                        } else {
                            5i32
                        }
                    } else {
                        4i32
                    }
                } else {
                    v262
                }
            } else {
                v94
            }
        } else {
            v91
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_self_test(v0: i32) -> i32 {
    closure0()(v0)
}
