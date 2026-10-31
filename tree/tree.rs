#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
enum US0 {
    US0_Ok(Vec<std::string::String>),
    US0_Error(std::io::Error),
}
#[derive(Clone)]
enum UH0 {
    UH0_ChildEnd,
    UH0_Child(i32, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_End,
    UH1_File(Rc<str>, Rc<str>, i64, Rc<UH1>),
    UH1_Folder(Rc<str>, Rc<str>, Rc<UH1>, Rc<UH1>),
}
fn ok_c1() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US0> = Rc::new(move |mut v0: Vec<std::string::String>| -> US0 {
        US0::US0_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method0() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US0> = Rc::new(move |mut v0: std::io::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method1() -> Rc<dyn Fn(std::io::Error) -> US0> {
    error_c2()
}
fn directory_exists_2(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn file_exists_3(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn cmp_at_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: i32 = (v1.clone().len() as i32);
        let mut v5: bool = v2 == v3;
        let mut v7: bool = if v5 {
            let mut v6: bool = v2 == v4;
            v6
        } else {
            false
        };
        if v7 {
            return 0i32;
        } else {
            if v5 {
                return -1i32;
            } else {
                let v8: bool = v2 == v4;
                if v8 {
                    return 1i32;
                } else {
                    let v9: u8 = v0.clone().as_bytes()[v2 as usize];
                    let v10: u8 = v1.clone().as_bytes()[v2 as usize];
                    let v11: bool = v9 == v10;
                    if v11 {
                        let v12: i32 = v2.wrapping_add(1i32);
                        (v0, v1, v2) = (v0.clone(), v1.clone(), v12);
                        continue;
                    } else {
                        let v14: bool = v9 < v10;
                        if v14 {
                            return -1i32;
                        } else {
                            return 1i32;
                        }
                    }
                }
            }
        }
    }
}
fn insert_5(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v2 {
        UH0::UH0_Child(v5, v6, v7) => {
            let mut v5: i32 = *v5;
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<UH0> = v7.clone();
            let v8: bool = v0 == v5;
            let v13: bool = if v8 {
                let v9: i32 = 0i32;
                let v10: i32 = cmp_at_6(v1.clone(), v6.clone(), v9);
                let v11: bool = v10 < 0i32;
                v11
            } else {
                let v12: bool = v0 > v5;
                v12
            };
            if v13 {
                Rc::new(UH0::UH0_Child(v0, v1.clone(), v2.clone()))
            } else {
                let v15: Rc<UH0> = insert_5(v0, v1.clone(), v7.clone());
                Rc::new(UH0::UH0_Child(v5, v6.clone(), v15.clone()))
            }
        }
        UH0::UH0_ChildEnd => {
            let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Child(v0, v1.clone(), v3.clone()))
        }
    }
}
fn sort_4(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Child(v2, v3, v4) => {
            let v2: i32 = *v2;
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let mut v5: Rc<UH0> = sort_4(v4.clone());
            insert_5(v2, v3.clone(), v5.clone())
        }
        UH0::UH0_ChildEnd => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) }
        }
    }
}
fn chain_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH1> {
    match &*v2 {
        UH0::UH0_Child(v4, v5, v6) => {
            let mut v4: i32 = *v4;
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            let mut v7: Rc<UH1> = chain_7(v0.clone(), v1.clone(), v6.clone());
            let v8: bool = v4 == 1i32;
            if v8 {
                let v10: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v5.as_ref()).display().to_string());
                let v11: i32 = (v1.clone().len() as i32);
                let v12: bool = v11 == 0i32;
                let v16: Rc<str> = if v12 {
                    v5.clone()
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v13)), v5));
                    v15.clone()
                };
                let v18: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v10).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                let mut v19: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method0();
                let mut v20: Rc<dyn Fn(std::io::Error) -> US0> = method1();
                let v21: US0 = match v18 { Ok(x) => v19(x), Err(e) => v20(e) };
                let v54: Rc<UH0> = match v21 {
                    US0::US0_Error(v51) => {
                        let v51: std::io::Error = v51;
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) }
                    }
                    US0::US0_Ok(v22) => {
                        let mut v22: Vec<std::string::String> = v22.clone();
                        let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) };
                        let v25: bool = true; let _vec_fold_ = v22.into_iter().fold(v23, |acc, x| { //;
                        let v27: Rc<UH0> = acc;
                        let v29: std::string::String = x;
                        let v31: Rc<str> = Rc::<str>::from(String::as_str(&v29));
                        let v33: Rc<str> = Rc::<str>::from(std::path::Path::new(v10.as_ref()).join(v31.as_ref()).display().to_string());
                        let mut v34: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v33);
                        let v35: bool = v34.is_ok();
                        let v36: bool = v35 == false;
                        let v39: bool = if v36 {
                            false
                        } else {
                            let mut v37: std::fs::Metadata = v34.unwrap();
                            let v38: bool = v37.file_type().is_symlink();
                            v38
                        };
                        let v46: Rc<UH0> = if v39 {
                            v27.clone()
                        } else {
                            let v40: bool = directory_exists_2(v33.clone());
                            if v40 {
                                Rc::new(UH0::UH0_Child(1i32, v31.clone(), v27.clone()))
                            } else {
                                let v42: bool = file_exists_3(v33.clone());
                                if v42 {
                                    Rc::new(UH0::UH0_Child(0i32, v31.clone(), v27.clone()))
                                } else {
                                    v27.clone()
                                }
                            }
                        };
                        let v48: bool = true; v46 });
                        let v50: Rc<UH0> = _vec_fold_;
                        v50.clone()
                    }
                };
                let v55: Rc<UH0> = sort_4(v54.clone());
                let v56: Rc<UH1> = chain_7(v10.clone(), v16.clone(), v55.clone());
                Rc::new(UH1::UH1_Folder(v5.clone(), v16.clone(), v56.clone(), v7.clone()))
            } else {
                let v59: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v5.as_ref()).display().to_string());
                let mut v60: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v59);
                let v61: bool = v60.is_ok();
                let v62: bool = v61 == false;
                let v66: i64 = if v62 {
                    0i64
                } else {
                    let v63: std::fs::Metadata = v60.unwrap();
                    let v64: u64 = v63.len();
                    let v65: i64 = (v64 as i64);
                    v65
                };
                Rc::new(UH1::UH1_File(v5.clone(), v1.clone(), v66, v7.clone()))
            }
        }
        UH0::UH0_ChildEnd => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_End); } CASE.with(|case| case.clone()) }
        }
    }
}
fn show_i64_9(mut v0: i64) -> Rc<str> {
    let v1: i64 = v0.wrapping_rem(10i64);
    let v2: bool = v1 == 0i64;
    let v32: Rc<str> = if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: bool = v1 == 1i64;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        } else {
            let mut v6: bool = v1 == 2i64;
            if v6 {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
                v7.clone()
            } else {
                let v8: bool = v1 == 3i64;
                if v8 {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                } else {
                    let v10: bool = v1 == 4i64;
                    if v10 {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                        v11.clone()
                    } else {
                        let v12: bool = v1 == 5i64;
                        if v12 {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5"); } LIT.with(|lit| lit.clone()) };
                            v13.clone()
                        } else {
                            let v14: bool = v1 == 6i64;
                            if v14 {
                                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
                                v15.clone()
                            } else {
                                let v16: bool = v1 == 7i64;
                                if v16 {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("7"); } LIT.with(|lit| lit.clone()) };
                                    v17.clone()
                                } else {
                                    let v18: bool = v1 == 8i64;
                                    if v18 {
                                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("8"); } LIT.with(|lit| lit.clone()) };
                                        v19.clone()
                                    } else {
                                        let mut v20: bool = v1 == 9i64;
                                        if v20 {
                                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("9"); } LIT.with(|lit| lit.clone()) };
                                            v21.clone()
                                        } else {
                                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                                            v22.clone()
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
    let v33: i64 = v0.wrapping_div(10i64);
    let mut v34: bool = v33 == 0i64;
    if v34 {
        v32.clone()
    } else {
        let v35: Rc<str> = show_i64_9(v33);
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v32));
        v36.clone()
    }
}
fn folder_bytes_10(mut v0: Rc<UH1>) -> i64 {
    match &*v0 {
        UH1::UH1_End => {
            0i64
        }
        UH1::UH1_File(v1, v2, v3, v4) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<str> = v2.clone();
            let v3: i64 = *v3;
            let mut v4: Rc<UH1> = v4.clone();
            let mut v5: i64 = folder_bytes_10(v4);
            let mut v6: i64 = v3.wrapping_add(v5);
            v6
        }
        UH1::UH1_Folder(v7, v8, v9, v10) => {
            let mut v7: Rc<str> = v7.clone();
            let v8: Rc<str> = v8.clone();
            let v9: Rc<UH1> = v9.clone();
            let v10: Rc<UH1> = v10.clone();
            let v11: i64 = folder_bytes_10(v9);
            let v12: i64 = folder_bytes_10(v10);
            let v13: i64 = v11.wrapping_add(v12);
            v13
        }
    }
}
fn html_chain_8(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_End => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        UH1::UH1_File(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let mut v4: i64 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let mut v6: i32 = (v3.clone().len() as i32);
            let mut v7: bool = v6 == 0i32;
            let v11: Rc<str> = if v7 {
                v2.clone()
            } else {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v8)), v2));
                v10.clone()
            };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196;"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <a href="); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196; <a href="); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v15)), v11)), v15));
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v2));
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a><span> ("); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
            let v24: bool = v4 > 1048576i64;
            let v73: Rc<str> = if v24 {
                let v25: i64 = v4.wrapping_mul(100i64);
                let v26: i64 = v25.wrapping_add(524288i64);
                let v27: i64 = v26.wrapping_div(1048576i64);
                let v28: i64 = v27.wrapping_div(100i64);
                let v29: Rc<str> = show_i64_9(v28);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                let v32: i64 = v27.wrapping_rem(100i64);
                let v33: Rc<str> = show_i64_9(v32);
                let mut v34: bool = v32 < 10i64;
                let mut v37: Rc<str> = if v34 {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v33));
                    v36.clone()
                } else {
                    v33.clone()
                };
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v37));
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
                v40.clone()
            } else {
                let v41: bool = v4 > 1024i64;
                if v41 {
                    let v42: i64 = v4.wrapping_mul(100i64);
                    let v43: i64 = v42.wrapping_add(512i64);
                    let v44: i64 = v43.wrapping_div(1024i64);
                    let v45: i64 = v44.wrapping_div(100i64);
                    let v46: Rc<str> = show_i64_9(v45);
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                    let v49: i64 = v44.wrapping_rem(100i64);
                    let v50: Rc<str> = show_i64_9(v49);
                    let v51: bool = v49 < 10i64;
                    let v54: Rc<str> = if v51 {
                        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v50));
                        v53.clone()
                    } else {
                        v50.clone()
                    };
                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v54));
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) };
                    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                    v57.clone()
                } else {
                    let v58: i64 = v4.wrapping_mul(100i64);
                    let v59: i64 = v58.wrapping_div(100i64);
                    let mut v60: Rc<str> = show_i64_9(v59);
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                    let v63: i64 = v58.wrapping_rem(100i64);
                    let v64: Rc<str> = show_i64_9(v63);
                    let v65: bool = v63 < 10i64;
                    let v68: Rc<str> = if v65 {
                        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                        let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v64));
                        v67.clone()
                    } else {
                        v64.clone()
                    };
                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v68));
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                    v71.clone()
                }
            };
            let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v73));
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")</span>"); } LIT.with(|lit| lit.clone()) };
            let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
            let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>"); } LIT.with(|lit| lit.clone()) };
            let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>"); } LIT.with(|lit| lit.clone()) };
            let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
            let v81: Rc<str> = html_chain_8(v5.clone());
            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
            v82.clone()
        }
        UH1::UH1_Folder(v83, v84, v85, v86) => {
            let v83: Rc<str> = v83.clone();
            let v84: Rc<str> = v84.clone();
            let v85: Rc<UH1> = v85.clone();
            let v86: Rc<UH1> = v86.clone();
            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open="); } LIT.with(|lit| lit.clone()) };
            let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\""); } LIT.with(|lit| lit.clone()) };
            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("true"); } LIT.with(|lit| lit.clone()) };
            let v92: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v89, v90)), v88));
            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("><summary>"); } LIT.with(|lit| lit.clone()) };
            let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
            let v95: i64 = folder_bytes_10(v85.clone());
            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128194;"); } LIT.with(|lit| lit.clone()) };
            let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <a href="); } LIT.with(|lit| lit.clone()) };
            let v101: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v96, v97)), v88)), v84)), v88));
            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v101, v102)), v83));
            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a><span> ("); } LIT.with(|lit| lit.clone()) };
            let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
            let v107: bool = v95 > 1048576i64;
            let v156: Rc<str> = if v107 {
                let v108: i64 = v95.wrapping_mul(100i64);
                let v109: i64 = v108.wrapping_add(524288i64);
                let v110: i64 = v109.wrapping_div(1048576i64);
                let v111: i64 = v110.wrapping_div(100i64);
                let v112: Rc<str> = show_i64_9(v111);
                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
                let v115: i64 = v110.wrapping_rem(100i64);
                let v116: Rc<str> = show_i64_9(v115);
                let v117: bool = v115 < 10i64;
                let v120: Rc<str> = if v117 {
                    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v116));
                    v119.clone()
                } else {
                    v116.clone()
                };
                let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v120));
                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) };
                let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
                v123.clone()
            } else {
                let v124: bool = v95 > 1024i64;
                if v124 {
                    let v125: i64 = v95.wrapping_mul(100i64);
                    let v126: i64 = v125.wrapping_add(512i64);
                    let v127: i64 = v126.wrapping_div(1024i64);
                    let v128: i64 = v127.wrapping_div(100i64);
                    let v129: Rc<str> = show_i64_9(v128);
                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                    let v132: i64 = v127.wrapping_rem(100i64);
                    let v133: Rc<str> = show_i64_9(v132);
                    let v134: bool = v132 < 10i64;
                    let v137: Rc<str> = if v134 {
                        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                        let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v133));
                        v136.clone()
                    } else {
                        v133.clone()
                    };
                    let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v137));
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) };
                    let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v139));
                    v140.clone()
                } else {
                    let v141: i64 = v95.wrapping_mul(100i64);
                    let v142: i64 = v141.wrapping_div(100i64);
                    let v143: Rc<str> = show_i64_9(v142);
                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v144));
                    let v146: i64 = v141.wrapping_rem(100i64);
                    let v147: Rc<str> = show_i64_9(v146);
                    let v148: bool = v146 < 10i64;
                    let v151: Rc<str> = if v148 {
                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                        let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v147));
                        v150.clone()
                    } else {
                        v147.clone()
                    };
                    let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v151));
                    let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) };
                    let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v153));
                    v154.clone()
                }
            };
            let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v156));
            let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")</span>"); } LIT.with(|lit| lit.clone()) };
            let v160: Rc<str> = Rc::<str>::from(format!("{}{}", v94, Rc::<str>::from(format!("{}{}", v157, v158))));
            let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</summary><div>"); } LIT.with(|lit| lit.clone()) };
            let v162: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v161));
            let v163: Rc<str> = html_chain_8(v85.clone());
            let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
            let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div></details>"); } LIT.with(|lit| lit.clone()) };
            let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v165));
            let v167: Rc<str> = html_chain_8(v86.clone());
            let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
            v168.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v3: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v4: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method0();
        let mut v5: Rc<dyn Fn(std::io::Error) -> US0> = method1();
        let mut v6: US0 = match v3 { Ok(x) => v4(x), Err(e) => v5(e) };
        let v39: Rc<UH0> = match v6 {
            US0::US0_Error(v36) => {
                let v36: std::io::Error = v36;
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) }
            }
            US0::US0_Ok(v7) => {
                let mut v7: Vec<std::string::String> = v7.clone();
                let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) };
                let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
                let v12: Rc<UH0> = acc;
                let v14: std::string::String = x;
                let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
                let v18: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v16.as_ref()).display().to_string());
                let mut v19: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v18);
                let mut v20: bool = v19.is_ok();
                let v21: bool = v20 == false;
                let v24: bool = if v21 {
                    false
                } else {
                    let mut v22: std::fs::Metadata = v19.unwrap();
                    let v23: bool = v22.file_type().is_symlink();
                    v23
                };
                let v31: Rc<UH0> = if v24 {
                    v12.clone()
                } else {
                    let v25: bool = directory_exists_2(v18.clone());
                    if v25 {
                        Rc::new(UH0::UH0_Child(1i32, v16.clone(), v12.clone()))
                    } else {
                        let v27: bool = file_exists_3(v18.clone());
                        if v27 {
                            Rc::new(UH0::UH0_Child(0i32, v16.clone(), v12.clone()))
                        } else {
                            v12.clone()
                        }
                    }
                };
                let v33: bool = true; v31 });
                let v35: Rc<UH0> = _vec_fold_;
                v35.clone()
            }
        };
        let v40: Rc<UH0> = sort_4(v39.clone());
        let v41: Rc<UH1> = chain_7(v0.clone(), v1.clone(), v40.clone());
        html_chain_8(v41.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> i64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i64> = Rc::new(move |mut v0: Rc<str>| -> i64 {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v3: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v4: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method0();
        let mut v5: Rc<dyn Fn(std::io::Error) -> US0> = method1();
        let mut v6: US0 = match v3 { Ok(x) => v4(x), Err(e) => v5(e) };
        let v39: Rc<UH0> = match v6 {
            US0::US0_Error(v36) => {
                let v36: std::io::Error = v36;
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) }
            }
            US0::US0_Ok(v7) => {
                let mut v7: Vec<std::string::String> = v7.clone();
                let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) };
                let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
                let v12: Rc<UH0> = acc;
                let v14: std::string::String = x;
                let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
                let v18: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v16.as_ref()).display().to_string());
                let mut v19: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v18);
                let mut v20: bool = v19.is_ok();
                let v21: bool = v20 == false;
                let v24: bool = if v21 {
                    false
                } else {
                    let mut v22: std::fs::Metadata = v19.unwrap();
                    let v23: bool = v22.file_type().is_symlink();
                    v23
                };
                let v31: Rc<UH0> = if v24 {
                    v12.clone()
                } else {
                    let v25: bool = directory_exists_2(v18.clone());
                    if v25 {
                        Rc::new(UH0::UH0_Child(1i32, v16.clone(), v12.clone()))
                    } else {
                        let v27: bool = file_exists_3(v18.clone());
                        if v27 {
                            Rc::new(UH0::UH0_Child(0i32, v16.clone(), v12.clone()))
                        } else {
                            v12.clone()
                        }
                    }
                };
                let v33: bool = true; v31 });
                let v35: Rc<UH0> = _vec_fold_;
                v35.clone()
            }
        };
        let v40: Rc<UH0> = sort_4(v39.clone());
        let v41: Rc<UH1> = chain_7(v0.clone(), v1.clone(), v40.clone());
        folder_bytes_10(v41.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(i64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i64) -> Rc<str>> = Rc::new(move |mut v0: i64| -> Rc<str> {
        let v1: bool = v0 > 1048576i64;
        if v1 {
            let v2: i64 = v0.wrapping_mul(100i64);
            let v3: i64 = v2.wrapping_add(524288i64);
            let mut v4: i64 = v3.wrapping_div(1048576i64);
            let mut v5: i64 = v4.wrapping_div(100i64);
            let mut v6: Rc<str> = show_i64_9(v5);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let v9: i64 = v4.wrapping_rem(100i64);
            let v10: Rc<str> = show_i64_9(v9);
            let v11: bool = v9 < 10i64;
            let v14: Rc<str> = if v11 {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v10));
                v13.clone()
            } else {
                v10.clone()
            };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v14));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            v17.clone()
        } else {
            let v18: bool = v0 > 1024i64;
            if v18 {
                let mut v19: i64 = v0.wrapping_mul(100i64);
                let mut v20: i64 = v19.wrapping_add(512i64);
                let v21: i64 = v20.wrapping_div(1024i64);
                let mut v22: i64 = v21.wrapping_div(100i64);
                let v23: Rc<str> = show_i64_9(v22);
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                let v26: i64 = v21.wrapping_rem(100i64);
                let v27: Rc<str> = show_i64_9(v26);
                let v28: bool = v26 < 10i64;
                let v31: Rc<str> = if v28 {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v27));
                    v30.clone()
                } else {
                    v27.clone()
                };
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v31));
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) };
                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
                v34.clone()
            } else {
                let v35: i64 = v0.wrapping_mul(100i64);
                let v36: i64 = v35.wrapping_div(100i64);
                let mut v37: Rc<str> = show_i64_9(v36);
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                let v40: i64 = v35.wrapping_rem(100i64);
                let v41: Rc<str> = show_i64_9(v40);
                let v42: bool = v40 < 10i64;
                let v45: Rc<str> = if v42 {
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v41));
                    v44.clone()
                } else {
                    v41.clone()
                };
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v45));
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                v48.clone()
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_tree_html(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn dtl_tree_bytes(v0: &str) -> i64 {
    closure3()(Rc::<str>::from(v0))
}
pub fn dtl_size_label(v0: i64) -> Rc<str> {
    closure4()(v0)
}
