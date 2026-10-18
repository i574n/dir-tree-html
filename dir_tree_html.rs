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
#[derive(Clone)]
enum US0 {
    US0_MdNone,
    US0_MdParagraph,
    US0_MdList,
    US0_MdCode,
}
fn show_i64_0(mut v0: i64) -> Rc<str> {
    let mut v1: i64 = v0.wrapping_rem(10i64);
    let mut v2: bool = v1 == 0i64;
    let mut v32: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
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
                let mut v8: bool = v1 == 3i64;
                if v8 {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                } else {
                    let mut v10: bool = v1 == 4i64;
                    if v10 {
                        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                        v11.clone()
                    } else {
                        let mut v12: bool = v1 == 5i64;
                        if v12 {
                            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5"); } LIT.with(|lit| lit.clone()) };
                            v13.clone()
                        } else {
                            let mut v14: bool = v1 == 6i64;
                            if v14 {
                                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
                                v15.clone()
                            } else {
                                let mut v16: bool = v1 == 7i64;
                                if v16 {
                                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("7"); } LIT.with(|lit| lit.clone()) };
                                    v17.clone()
                                } else {
                                    let mut v18: bool = v1 == 8i64;
                                    if v18 {
                                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("8"); } LIT.with(|lit| lit.clone()) };
                                        v19.clone()
                                    } else {
                                        let mut v20: bool = v1 == 9i64;
                                        if v20 {
                                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("9"); } LIT.with(|lit| lit.clone()) };
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
    let mut v33: i64 = v0.wrapping_div(10i64);
    let mut v34: bool = v33 == 0i64;
    if v34 {
        v32.clone()
    } else {
        let mut v35: Rc<str> = show_i64_0(v33);
        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35.clone(), v32.clone()));
        v36.clone()
    }
}
fn times_2(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let mut v2: bool = v1 <= 0i64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: i64 = v1.wrapping_sub(1i64);
        let mut v5: Rc<str> = times_2(v0.clone(), v4);
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), v5.clone()));
        v6.clone()
    }
}
fn last_sep_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3;
        if v4 {
            return v2;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'/';
            let mut v9: i32 = if v6 {
                v1
            } else {
                let mut v7: bool = v5 == b'\\';
                if v7 {
                    v1
                } else {
                    v2
                }
            };
            let mut v10: i32 = v1.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v10, v9);
            continue;
        }
    }
}
fn lay_1(mut v0: Rc<str>, mut v1: i64) -> i32 {
    loop {
        let mut v2: bool = v1 < 0i64;
        if v2 {
            return 0i32;
        } else {
            let mut v4: i32 = i32::from(std::fs::create_dir_all(v0.as_ref()).is_err());
            let mut v5: bool = v1 == 0i64;
            let mut v35: Rc<str> = if v5 {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            } else {
                let mut v7: bool = v1 == 1i64;
                if v7 {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                } else {
                    let mut v9: bool = v1 == 2i64;
                    if v9 {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
                        v10.clone()
                    } else {
                        let mut v11: bool = v1 == 3i64;
                        if v11 {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        } else {
                            let mut v13: bool = v1 == 4i64;
                            if v13 {
                                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                                v14.clone()
                            } else {
                                let mut v15: bool = v1 == 5i64;
                                if v15 {
                                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5"); } LIT.with(|lit| lit.clone()) };
                                    v16.clone()
                                } else {
                                    let mut v17: bool = v1 == 6i64;
                                    if v17 {
                                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
                                        v18.clone()
                                    } else {
                                        let mut v19: bool = v1 == 7i64;
                                        if v19 {
                                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("7"); } LIT.with(|lit| lit.clone()) };
                                            v20.clone()
                                        } else {
                                            let mut v21: bool = v1 == 8i64;
                                            if v21 {
                                                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("8"); } LIT.with(|lit| lit.clone()) };
                                                v22.clone()
                                            } else {
                                                let mut v23: bool = v1 == 9i64;
                                                if v23 {
                                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("9"); } LIT.with(|lit| lit.clone()) };
                                                    v24.clone()
                                                } else {
                                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
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
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v37.as_ref()).display().to_string());
            let mut v39: i64 = v1.wrapping_add(1i64);
            let mut v40: Rc<str> = times_2(v35.clone(), v39);
            let mut v41: i32 = 0i32;
            let mut v42: i32 = -1i32;
            let mut v43: i32 = last_sep_3(v38.clone(), v41, v42);
            let mut v44: bool = v43 == -1i32;
            let mut v55: Rc<str> = if v44 {
                let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v45.clone()
            } else {
                let mut v46: i32 = v43.wrapping_sub(1i32);
                let mut v47: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v46 as i64);
                let mut v48: i32 = (v47.clone().len() as i32);
                let mut v49: bool = v48 == 2i32;
                let mut v52: bool = if v49 {
                    let mut v50: u8 = v47.clone().as_bytes()[1i32 as usize];
                    let mut v51: bool = v50 == b':';
                    v51
                } else {
                    false
                };
                if v52 {
                    let mut v53: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v43 as i64);
                    v53.clone()
                } else {
                    v47.clone()
                }
            };
            let mut v56: i32 = (v55.clone().len() as i32);
            let mut v57: bool = v56 == 0i32;
            let mut v60: i32 = if v57 {
                0i32
            } else {
                let mut v59: i32 = i32::from(std::fs::create_dir_all(v55.as_ref()).is_err());
                v59
            };
            let mut v61: bool = v60 == 0i32;
            let mut v64: i32 = if v61 {
                let mut v63: i32 = i32::from(std::fs::write(v38.as_ref(), v40.as_ref().as_bytes()).is_err());
                v63
            } else {
                v60
            };
            let mut v66: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v35.as_ref()).display().to_string());
            let mut v67: i64 = v1.wrapping_sub(1i64);
            (v0, v1) = (v66.clone(), v67);
            continue;
        }
    }
}
fn find_nl_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = v4 == b'\n';
            if v5 {
                return v1;
            } else {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            }
        }
    }
}
fn parse_digits_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v1 == v2;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v1.wrapping_add(1i32);
            let mut v6: i32 = v3.wrapping_mul(10i32);
            let mut v7: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v8: bool = v7 == b'0';
            let mut v27: i32 = if v8 {
                0i32
            } else {
                let mut v9: bool = v7 == b'1';
                if v9 {
                    1i32
                } else {
                    let mut v10: bool = v7 == b'2';
                    if v10 {
                        2i32
                    } else {
                        let mut v11: bool = v7 == b'3';
                        if v11 {
                            3i32
                        } else {
                            let mut v12: bool = v7 == b'4';
                            if v12 {
                                4i32
                            } else {
                                let mut v13: bool = v7 == b'5';
                                if v13 {
                                    5i32
                                } else {
                                    let mut v14: bool = v7 == b'6';
                                    if v14 {
                                        6i32
                                    } else {
                                        let mut v15: bool = v7 == b'7';
                                        if v15 {
                                            7i32
                                        } else {
                                            let mut v16: bool = v7 == b'8';
                                            if v16 {
                                                8i32
                                            } else {
                                                let mut v17: bool = v7 == b'9';
                                                if v17 {
                                                    9i32
                                                } else {
                                                    0i32
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
            let mut v28: i32 = v6.wrapping_add(v27);
            (v0, v1, v2, v3) = (v0.clone(), v5, v2, v28);
            continue;
        }
    }
}
fn parse_4(mut v0: Rc<str>, mut v1: i32) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v1 == v2;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: i32 = v1.wrapping_add(2i32);
        let mut v6: i32 = find_nl_5(v0.clone(), v5);
        let mut v7: i32 = 0i32;
        let mut v8: i32 = parse_digits_6(v0.clone(), v5, v6, v7);
        let mut v9: i32 = v6.wrapping_add(1i32);
        let mut v10: i32 = v9.wrapping_add(v8);
        let mut v11: i32 = v10.wrapping_sub(1i32);
        let mut v12: Rc<str> = string_slice(&v0.clone(), v9 as i64, v11 as i64);
        let mut v13: u8 = v0.clone().as_bytes()[v1 as usize];
        let mut v14: bool = v13 == b'1';
        let mut v15: i32 = if v14 {
            1i32
        } else {
            0i32
        };
        let mut v16: Rc<UH0> = parse_4(v0.clone(), v10);
        Rc::new(UH0::UH0_Child(v15, v12.clone(), v16.clone()))
    }
}
fn cmp_at_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
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
                let mut v8: bool = v2 == v4;
                if v8 {
                    return 1i32;
                } else {
                    let mut v9: u8 = v0.clone().as_bytes()[v2 as usize];
                    let mut v10: u8 = v1.clone().as_bytes()[v2 as usize];
                    let mut v11: bool = v9 == v10;
                    if v11 {
                        let mut v12: i32 = v2.wrapping_add(1i32);
                        (v0, v1, v2) = (v0.clone(), v1.clone(), v12);
                        continue;
                    } else {
                        let mut v14: bool = v9 < v10;
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
fn insert_8(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v2 {
        UH0::UH0_Child(v5, v6, v7) => {
            let mut v5: i32 = *v5;
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<UH0> = v7.clone();
            let mut v8: bool = v0 == v5;
            let mut v13: bool = if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: i32 = cmp_at_9(v1.clone(), v6.clone(), v9);
                let mut v11: bool = v10 < 0i32;
                v11
            } else {
                let mut v12: bool = v0 > v5;
                v12
            };
            if v13 {
                Rc::new(UH0::UH0_Child(v0, v1.clone(), v2.clone()))
            } else {
                let mut v15: Rc<UH0> = insert_8(v0, v1.clone(), v7.clone());
                Rc::new(UH0::UH0_Child(v5, v6.clone(), v15.clone()))
            }
        }
        UH0::UH0_ChildEnd => {
            let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Child(v0, v1.clone(), v3.clone()))
        }
    }
}
fn sort_7(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Child(v2, v3, v4) => {
            let mut v2: i32 = *v2;
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let mut v5: Rc<UH0> = sort_7(v4.clone());
            insert_8(v2, v3.clone(), v5.clone())
        }
        UH0::UH0_ChildEnd => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_ChildEnd); } CASE.with(|case| case.clone()) }
        }
    }
}
fn chain_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH1> {
    match &*v2 {
        UH0::UH0_Child(v4, v5, v6) => {
            let mut v4: i32 = *v4;
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            let mut v7: Rc<UH1> = chain_10(v0.clone(), v1.clone(), v6.clone());
            let mut v8: bool = v4 == 1i32;
            if v8 {
                let mut v10: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v5.as_ref()).display().to_string());
                let mut v11: i32 = (v1.clone().len() as i32);
                let mut v12: bool = v11 == 0i32;
                let mut v15: Rc<str> = if v12 {
                    v5.clone()
                } else {
                    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v5.clone()));
                    v14.clone()
                };
                let mut v17: Rc<str> = { let mut out = String::new(); if let Ok(read) = std::fs::read_dir(v10.as_ref()) { for entry in read.flatten() { let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue }; let child = entry.path(); let Some(name) = child.file_name() else { continue }; let name = name.to_string_lossy(); if child.is_dir() { out.push(char::from(49)); } else if child.is_file() { out.push(char::from(48)); } else { continue }; out.push(char::from(32)); out.push_str(&name.len().to_string()); out.push(char::from(10)); out.push_str(&name); } } Rc::<str>::from(out) };
                let mut v18: i32 = 0i32;
                let mut v19: Rc<UH0> = parse_4(v17.clone(), v18);
                let mut v20: Rc<UH0> = sort_7(v19.clone());
                let mut v21: Rc<UH1> = chain_10(v10.clone(), v15.clone(), v20.clone());
                Rc::new(UH1::UH1_Folder(v5.clone(), v15.clone(), v21.clone(), v7.clone()))
            } else {
                let mut v24: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v5.as_ref()).display().to_string());
                let mut v26: i64 = std::fs::metadata(v24.as_ref()).map(|meta| meta.len() as i64).unwrap_or(0);
                Rc::new(UH1::UH1_File(v5.clone(), v1.clone(), v26, v7.clone()))
            }
        }
        UH0::UH0_ChildEnd => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_End); } CASE.with(|case| case.clone()) }
        }
    }
}
fn folder_bytes_12(mut v0: Rc<UH1>) -> i64 {
    match &*v0 {
        UH1::UH1_End => {
            0i64
        }
        UH1::UH1_File(v1, v2, v3, v4) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let mut v3: i64 = *v3;
            let mut v4: Rc<UH1> = v4.clone();
            let mut v5: i64 = folder_bytes_12(v4.clone());
            let mut v6: i64 = v3.wrapping_add(v5);
            v6
        }
        UH1::UH1_Folder(v7, v8, v9, v10) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = v8.clone();
            let mut v9: Rc<UH1> = v9.clone();
            let mut v10: Rc<UH1> = v10.clone();
            let mut v11: i64 = folder_bytes_12(v9.clone());
            let mut v12: i64 = folder_bytes_12(v10.clone());
            let mut v13: i64 = v11.wrapping_add(v12);
            v13
        }
    }
}
fn html_chain_11(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_End => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        UH1::UH1_File(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: i64 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let mut v6: i32 = (v3.clone().len() as i32);
            let mut v7: bool = v6 == 0i32;
            let mut v10: Rc<str> = if v7 {
                v2.clone()
            } else {
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8.clone(), v2.clone()));
                v9.clone()
            };
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196; <a href=\""); } LIT.with(|lit| lit.clone()) }, v10.clone()));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) }));
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v2.clone()));
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a><span> ("); } LIT.with(|lit| lit.clone()) }));
            let mut v16: bool = v4 > 1048576i64;
            let mut v56: Rc<str> = if v16 {
                let mut v17: i64 = v4.wrapping_mul(100i64);
                let mut v18: i64 = v17.wrapping_add(524288i64);
                let mut v19: i64 = v18.wrapping_div(1048576i64);
                let mut v20: i64 = v19.wrapping_div(100i64);
                let mut v21: Rc<str> = show_i64_0(v20);
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                let mut v23: i64 = v19.wrapping_rem(100i64);
                let mut v24: Rc<str> = show_i64_0(v23);
                let mut v25: bool = v23 < 10i64;
                let mut v27: Rc<str> = if v25 {
                    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v24.clone()));
                    v26.clone()
                } else {
                    v24.clone()
                };
                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v22.clone(), v27.clone()));
                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) }));
                v29.clone()
            } else {
                let mut v30: bool = v4 > 1024i64;
                if v30 {
                    let mut v31: i64 = v4.wrapping_mul(100i64);
                    let mut v32: i64 = v31.wrapping_add(512i64);
                    let mut v33: i64 = v32.wrapping_div(1024i64);
                    let mut v34: i64 = v33.wrapping_div(100i64);
                    let mut v35: Rc<str> = show_i64_0(v34);
                    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                    let mut v37: i64 = v33.wrapping_rem(100i64);
                    let mut v38: Rc<str> = show_i64_0(v37);
                    let mut v39: bool = v37 < 10i64;
                    let mut v41: Rc<str> = if v39 {
                        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v38.clone()));
                        v40.clone()
                    } else {
                        v38.clone()
                    };
                    let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v36.clone(), v41.clone()));
                    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) }));
                    v43.clone()
                } else {
                    let mut v44: i64 = v4.wrapping_mul(100i64);
                    let mut v45: i64 = v44.wrapping_div(100i64);
                    let mut v46: Rc<str> = show_i64_0(v45);
                    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                    let mut v48: i64 = v44.wrapping_rem(100i64);
                    let mut v49: Rc<str> = show_i64_0(v48);
                    let mut v50: bool = v48 < 10i64;
                    let mut v52: Rc<str> = if v50 {
                        let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v49.clone()));
                        v51.clone()
                    } else {
                        v49.clone()
                    };
                    let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v47.clone(), v52.clone()));
                    let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) }));
                    v54.clone()
                }
            };
            let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v15.clone(), v56.clone()));
            let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")</span>"); } LIT.with(|lit| lit.clone()) }));
            let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>"); } LIT.with(|lit| lit.clone()) }, v58.clone()));
            let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>"); } LIT.with(|lit| lit.clone()) }));
            let mut v61: Rc<str> = html_chain_11(v5.clone());
            let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60.clone(), v61.clone()));
            v62.clone()
        }
        UH1::UH1_Folder(v63, v64, v65, v66) => {
            let mut v63: Rc<str> = v63.clone();
            let mut v64: Rc<str> = v64.clone();
            let mut v65: Rc<UH1> = v65.clone();
            let mut v66: Rc<UH1> = v66.clone();
            let mut v67: i64 = folder_bytes_12(v65.clone());
            let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128194; <a href=\""); } LIT.with(|lit| lit.clone()) }, v64.clone()));
            let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) }));
            let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70.clone(), v63.clone()));
            let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a><span> ("); } LIT.with(|lit| lit.clone()) }));
            let mut v73: bool = v67 > 1048576i64;
            let mut v113: Rc<str> = if v73 {
                let mut v74: i64 = v67.wrapping_mul(100i64);
                let mut v75: i64 = v74.wrapping_add(524288i64);
                let mut v76: i64 = v75.wrapping_div(1048576i64);
                let mut v77: i64 = v76.wrapping_div(100i64);
                let mut v78: Rc<str> = show_i64_0(v77);
                let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                let mut v80: i64 = v76.wrapping_rem(100i64);
                let mut v81: Rc<str> = show_i64_0(v80);
                let mut v82: bool = v80 < 10i64;
                let mut v84: Rc<str> = if v82 {
                    let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v81.clone()));
                    v83.clone()
                } else {
                    v81.clone()
                };
                let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v79.clone(), v84.clone()));
                let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) }));
                v86.clone()
            } else {
                let mut v87: bool = v67 > 1024i64;
                if v87 {
                    let mut v88: i64 = v67.wrapping_mul(100i64);
                    let mut v89: i64 = v88.wrapping_add(512i64);
                    let mut v90: i64 = v89.wrapping_div(1024i64);
                    let mut v91: i64 = v90.wrapping_div(100i64);
                    let mut v92: Rc<str> = show_i64_0(v91);
                    let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v92.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                    let mut v94: i64 = v90.wrapping_rem(100i64);
                    let mut v95: Rc<str> = show_i64_0(v94);
                    let mut v96: bool = v94 < 10i64;
                    let mut v98: Rc<str> = if v96 {
                        let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v95.clone()));
                        v97.clone()
                    } else {
                        v95.clone()
                    };
                    let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v93.clone(), v98.clone()));
                    let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) }));
                    v100.clone()
                } else {
                    let mut v101: i64 = v67.wrapping_mul(100i64);
                    let mut v102: i64 = v101.wrapping_div(100i64);
                    let mut v103: Rc<str> = show_i64_0(v102);
                    let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                    let mut v105: i64 = v101.wrapping_rem(100i64);
                    let mut v106: Rc<str> = show_i64_0(v105);
                    let mut v107: bool = v105 < 10i64;
                    let mut v109: Rc<str> = if v107 {
                        let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v106.clone()));
                        v108.clone()
                    } else {
                        v106.clone()
                    };
                    let mut v110: Rc<str> = Rc::<str>::from(format!("{}{}", v104.clone(), v109.clone()));
                    let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) }));
                    v111.clone()
                }
            };
            let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v72.clone(), v113.clone()));
            let mut v115: Rc<str> = Rc::<str>::from(format!("{}{}", v114.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")</span>"); } LIT.with(|lit| lit.clone()) }));
            let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>"); } LIT.with(|lit| lit.clone()) }, v115.clone()));
            let mut v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</summary><div>"); } LIT.with(|lit| lit.clone()) }));
            let mut v118: Rc<str> = html_chain_11(v65.clone());
            let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117.clone(), v118.clone()));
            let mut v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div></details>"); } LIT.with(|lit| lit.clone()) }));
            let mut v121: Rc<str> = html_chain_11(v66.clone());
            let mut v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120.clone(), v121.clone()));
            v122.clone()
        }
    }
}
fn starts_at_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v1.clone().len() as i32);
        let mut v4: bool = v2 == v3;
        if v4 {
            return true;
        } else {
            let mut v5: i32 = (v0.clone().len() as i32);
            let mut v6: bool = v2 == v5;
            if v6 {
                return false;
            } else {
                let mut v7: u8 = v0.clone().as_bytes()[v2 as usize];
                let mut v8: u8 = v1.clone().as_bytes()[v2 as usize];
                let mut v9: bool = v7 == v8;
                if v9 {
                    let mut v10: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v1.clone(), v10);
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn escape_from_15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'&';
            let mut v18: Rc<str> = if v6 {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&amp;"); } LIT.with(|lit| lit.clone()) };
                v7.clone()
            } else {
                let mut v8: bool = v5 == b'<';
                if v8 {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&lt;"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                } else {
                    let mut v10: bool = v5 == b'>';
                    if v10 {
                        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&gt;"); } LIT.with(|lit| lit.clone()) };
                        v11.clone()
                    } else {
                        let mut v12: bool = v5 == b'"';
                        if v12 {
                            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&quot;"); } LIT.with(|lit| lit.clone()) };
                            v13.clone()
                        } else {
                            let mut v14: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                            v14.clone()
                        }
                    }
                }
            };
            let mut v19: i32 = v1.wrapping_add(1i32);
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v18.clone()));
            (v0, v1, v2) = (v0.clone(), v19, v20.clone());
            continue;
        }
    }
}
fn count_hashes_16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = v4 == b'#';
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn blank_from_17(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 == v2;
        if v3 {
            return true;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = v4 == b' ';
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn find_char_from_19(mut v0: Rc<str>, mut v1: u8, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v2 == v3;
        if v4 {
            return -1i32;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v6: bool = v5 == v1;
            if v6 {
                return v2;
            } else {
                let mut v7: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v7);
                continue;
            }
        }
    }
}
fn inline_from_18(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'`';
            if v6 {
                let mut v7: i32 = v1.wrapping_add(1i32);
                let mut v8: i32 = find_char_from_19(v0.clone(), v5, v7);
                let mut v9: bool = v8 == -1i32;
                if v9 {
                    let mut v10: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                    let mut v11: i32 = 0i32;
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<str> = escape_from_15(v10.clone(), v11, v12.clone());
                    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v13.clone()));
                    (v0, v1, v2) = (v0.clone(), v7, v14.clone());
                    continue;
                } else {
                    let mut v16: i32 = v8.wrapping_add(1i32);
                    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<code>"); } LIT.with(|lit| lit.clone()) }));
                    let mut v18: i32 = v8.wrapping_sub(1i32);
                    let mut v19: Rc<str> = string_slice(&v0.clone(), v7 as i64, v18 as i64);
                    let mut v20: i32 = 0i32;
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = escape_from_15(v19.clone(), v20, v21.clone());
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v22.clone()));
                    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code>"); } LIT.with(|lit| lit.clone()) }));
                    (v0, v1, v2) = (v0.clone(), v16, v24.clone());
                    continue;
                }
            } else {
                let mut v27: bool = v5 == b'[';
                if v27 {
                    let mut v28: u8 = b']';
                    let mut v29: i32 = v1.wrapping_add(1i32);
                    let mut v30: i32 = find_char_from_19(v0.clone(), v28, v29);
                    let mut v31: bool = v30 == -1i32;
                    if v31 {
                        let mut v32: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                        let mut v33: i32 = 0i32;
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v35: Rc<str> = escape_from_15(v32.clone(), v33, v34.clone());
                        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v35.clone()));
                        (v0, v1, v2) = (v0.clone(), v29, v36.clone());
                        continue;
                    } else {
                        let mut v38: i32 = v30.wrapping_add(1i32);
                        let mut v39: bool = v38 == v3;
                        if v39 {
                            let mut v40: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                            let mut v41: i32 = 0i32;
                            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v43: Rc<str> = escape_from_15(v40.clone(), v41, v42.clone());
                            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v43.clone()));
                            (v0, v1, v2) = (v0.clone(), v29, v44.clone());
                            continue;
                        } else {
                            let mut v46: u8 = v0.clone().as_bytes()[v38 as usize];
                            let mut v47: bool = v46 == b'(';
                            if v47 {
                                let mut v48: u8 = b')';
                                let mut v49: i32 = v30.wrapping_add(2i32);
                                let mut v50: i32 = find_char_from_19(v0.clone(), v48, v49);
                                let mut v51: bool = v50 == -1i32;
                                if v51 {
                                    let mut v52: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                                    let mut v53: i32 = 0i32;
                                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v55: Rc<str> = escape_from_15(v52.clone(), v53, v54.clone());
                                    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v55.clone()));
                                    (v0, v1, v2) = (v0.clone(), v29, v56.clone());
                                    continue;
                                } else {
                                    let mut v58: i32 = v30.wrapping_sub(1i32);
                                    let mut v59: Rc<str> = string_slice(&v0.clone(), v29 as i64, v58 as i64);
                                    let mut v60: i32 = v50.wrapping_sub(1i32);
                                    let mut v61: Rc<str> = string_slice(&v0.clone(), v49 as i64, v60 as i64);
                                    let mut v62: i32 = v50.wrapping_add(1i32);
                                    let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) }));
                                    let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                                    let mut v65: i32 = 0i32;
                                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v67: Rc<str> = escape_from_15(v61.clone(), v65, v66.clone());
                                    let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v64.clone(), v67.clone()));
                                    let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                                    let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) }));
                                    let mut v71: i32 = 0i32;
                                    let mut v72: Rc<str> = escape_from_15(v59.clone(), v71, v66.clone());
                                    let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v70.clone(), v72.clone()));
                                    let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v73.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) }));
                                    (v0, v1, v2) = (v0.clone(), v62, v74.clone());
                                    continue;
                                }
                            } else {
                                let mut v77: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                                let mut v78: i32 = 0i32;
                                let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v80: Rc<str> = escape_from_15(v77.clone(), v78, v79.clone());
                                let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v80.clone()));
                                (v0, v1, v2) = (v0.clone(), v29, v81.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    let mut v86: i32 = v1.wrapping_add(1i32);
                    let mut v87: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                    let mut v88: i32 = 0i32;
                    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v90: Rc<str> = escape_from_15(v87.clone(), v88, v89.clone());
                    let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v90.clone()));
                    (v0, v1, v2) = (v0.clone(), v86, v91.clone());
                    continue;
                }
            }
        }
    }
}
fn markdown_lines_13(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let mut v5: bool = v1 > v4;
        if v5 {
            let mut v13: Rc<str> = match &v2 {
                US0::US0_MdCode => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
                US0::US0_MdList => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US0::US0_MdNone => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US0::US0_MdParagraph => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
            };
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v13.clone()));
            return v14.clone();
        } else {
            let mut v15: i32 = find_nl_5(v0.clone(), v1);
            let mut v16: i32 = v15.wrapping_sub(1i32);
            let mut v17: Rc<str> = string_slice(&v0.clone(), v1 as i64, v16 as i64);
            let mut v18: i32 = v15.wrapping_add(1i32);
            match &v2 {
                US0::US0_MdCode => {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                    let mut v20: i32 = 0i32;
                    let mut v21: bool = starts_at_14(v17.clone(), v19.clone(), v20);
                    if v21 {
                        let mut v22: US0 = US0::US0_MdNone;
                        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) }));
                        (v0, v1, v2, v3) = (v0.clone(), v18, v22.clone(), v23.clone());
                        continue;
                    } else {
                        let mut v25: US0 = US0::US0_MdCode;
                        let mut v26: i32 = 0i32;
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v28: Rc<str> = escape_from_15(v17.clone(), v26, v27.clone());
                        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v28.clone()));
                        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        (v0, v1, v2, v3) = (v0.clone(), v18, v25.clone(), v30.clone());
                        continue;
                    }
                }
                _ => {
                    let mut v33: i32 = 0i32;
                    let mut v34: i32 = count_hashes_16(v17.clone(), v33);
                    let mut v35: bool = v34 == 0i32;
                    let mut v44: i32 = if v35 {
                        0i32
                    } else {
                        let mut v36: bool = v34 > 6i32;
                        if v36 {
                            0i32
                        } else {
                            let mut v37: i32 = (v17.clone().len() as i32);
                            let mut v38: bool = v34 == v37;
                            if v38 {
                                0i32
                            } else {
                                let mut v39: u8 = v17.clone().as_bytes()[v34 as usize];
                                let mut v40: bool = v39 == b' ';
                                if v40 {
                                    v34
                                } else {
                                    0i32
                                }
                            }
                        }
                    };
                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                    let mut v46: i32 = 0i32;
                    let mut v47: bool = starts_at_14(v17.clone(), v45.clone(), v46);
                    if v47 {
                        let mut v48: US0 = US0::US0_MdCode;
                        let mut v54: Rc<str> = match &v2 {
                            US0::US0_MdList => {
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US0::US0_MdNone => {
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                            US0::US0_MdParagraph => {
                                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v54.clone()));
                        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) }));
                        (v0, v1, v2, v3) = (v0.clone(), v18, v48.clone(), v56.clone());
                        continue;
                    } else {
                        let mut v58: i32 = 0i32;
                        let mut v59: bool = blank_from_17(v17.clone(), v58);
                        if v59 {
                            let mut v60: US0 = US0::US0_MdNone;
                            let mut v66: Rc<str> = match &v2 {
                                US0::US0_MdList => {
                                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                    v63.clone()
                                }
                                US0::US0_MdNone => {
                                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v61.clone()
                                }
                                US0::US0_MdParagraph => {
                                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                    v62.clone()
                                }
                                _ => unreachable!(),
                            };
                            let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v66.clone()));
                            (v0, v1, v2, v3) = (v0.clone(), v18, v60.clone(), v67.clone());
                            continue;
                        } else {
                            let mut v69: bool = v44 > 0i32;
                            if v69 {
                                let mut v70: bool = v44 == 1i32;
                                let mut v85: Rc<str> = if v70 {
                                    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1"); } LIT.with(|lit| lit.clone()) };
                                    v71.clone()
                                } else {
                                    let mut v72: bool = v44 == 2i32;
                                    if v72 {
                                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h2"); } LIT.with(|lit| lit.clone()) };
                                        v73.clone()
                                    } else {
                                        let mut v74: bool = v44 == 3i32;
                                        if v74 {
                                            let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h3"); } LIT.with(|lit| lit.clone()) };
                                            v75.clone()
                                        } else {
                                            let mut v76: bool = v44 == 4i32;
                                            if v76 {
                                                let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h4"); } LIT.with(|lit| lit.clone()) };
                                                v77.clone()
                                            } else {
                                                let mut v78: bool = v44 == 5i32;
                                                if v78 {
                                                    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h5"); } LIT.with(|lit| lit.clone()) };
                                                    v79.clone()
                                                } else {
                                                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h6"); } LIT.with(|lit| lit.clone()) };
                                                    v80.clone()
                                                }
                                            }
                                        }
                                    }
                                };
                                let mut v86: i32 = v44.wrapping_add(1i32);
                                let mut v87: i32 = (v17.clone().len() as i32);
                                let mut v88: i32 = v87.wrapping_sub(1i32);
                                let mut v89: Rc<str> = string_slice(&v17.clone(), v86 as i64, v88 as i64);
                                let mut v90: US0 = US0::US0_MdNone;
                                let mut v96: Rc<str> = match &v2 {
                                    US0::US0_MdList => {
                                        let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                        v93.clone()
                                    }
                                    US0::US0_MdNone => {
                                        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v91.clone()
                                    }
                                    US0::US0_MdParagraph => {
                                        let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                        v92.clone()
                                    }
                                    _ => unreachable!(),
                                };
                                let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v96.clone()));
                                let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) }));
                                let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98.clone(), v85.clone()));
                                let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) }));
                                let mut v101: i32 = 0i32;
                                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v103: Rc<str> = inline_from_18(v89.clone(), v101, v102.clone());
                                let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v100.clone(), v103.clone()));
                                let mut v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) }));
                                let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105.clone(), v85.clone()));
                                let mut v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">\n"); } LIT.with(|lit| lit.clone()) }));
                                (v0, v1, v2, v3) = (v0.clone(), v18, v90.clone(), v107.clone());
                                continue;
                            } else {
                                let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("- "); } LIT.with(|lit| lit.clone()) };
                                let mut v110: i32 = 0i32;
                                let mut v111: bool = starts_at_14(v17.clone(), v109.clone(), v110);
                                if v111 {
                                    let mut v112: i32 = (v17.clone().len() as i32);
                                    let mut v113: i32 = v112.wrapping_sub(1i32);
                                    let mut v114: Rc<str> = string_slice(&v17.clone(), 2i32 as i64, v113 as i64);
                                    let mut v115: i32 = 0i32;
                                    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v117: Rc<str> = inline_from_18(v114.clone(), v115, v116.clone());
                                    let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) }, v117.clone()));
                                    let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) }));
                                    match &v2 {
                                        US0::US0_MdList => {
                                            let mut v120: US0 = US0::US0_MdList;
                                            let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v119.clone()));
                                            (v0, v1, v2, v3) = (v0.clone(), v18, v120.clone(), v121.clone());
                                            continue;
                                        }
                                        _ => {
                                            let mut v123: US0 = US0::US0_MdList;
                                            let mut v126: Rc<str> = match &v2 {
                                                US0::US0_MdNone => {
                                                    v116.clone()
                                                }
                                                US0::US0_MdParagraph => {
                                                    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                                    v124.clone()
                                                }
                                                _ => unreachable!(),
                                            };
                                            let mut v127: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v126.clone()));
                                            let mut v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ul>\n"); } LIT.with(|lit| lit.clone()) }));
                                            let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128.clone(), v119.clone()));
                                            (v0, v1, v2, v3) = (v0.clone(), v18, v123.clone(), v129.clone());
                                            continue;
                                        }
                                    }
                                } else {
                                    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("* "); } LIT.with(|lit| lit.clone()) };
                                    let mut v133: i32 = 0i32;
                                    let mut v134: bool = starts_at_14(v17.clone(), v132.clone(), v133);
                                    if v134 {
                                        let mut v135: i32 = (v17.clone().len() as i32);
                                        let mut v136: i32 = v135.wrapping_sub(1i32);
                                        let mut v137: Rc<str> = string_slice(&v17.clone(), 2i32 as i64, v136 as i64);
                                        let mut v138: i32 = 0i32;
                                        let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v140: Rc<str> = inline_from_18(v137.clone(), v138, v139.clone());
                                        let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) }, v140.clone()));
                                        let mut v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) }));
                                        match &v2 {
                                            US0::US0_MdList => {
                                                let mut v143: US0 = US0::US0_MdList;
                                                let mut v144: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v142.clone()));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v143.clone(), v144.clone());
                                                continue;
                                            }
                                            _ => {
                                                let mut v146: US0 = US0::US0_MdList;
                                                let mut v149: Rc<str> = match &v2 {
                                                    US0::US0_MdNone => {
                                                        v139.clone()
                                                    }
                                                    US0::US0_MdParagraph => {
                                                        let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                                        v147.clone()
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                let mut v150: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v149.clone()));
                                                let mut v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ul>\n"); } LIT.with(|lit| lit.clone()) }));
                                                let mut v152: Rc<str> = Rc::<str>::from(format!("{}{}", v151.clone(), v142.clone()));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v146.clone(), v152.clone());
                                                continue;
                                            }
                                        }
                                    } else {
                                        match &v2 {
                                            US0::US0_MdParagraph => {
                                                let mut v155: US0 = US0::US0_MdParagraph;
                                                let mut v156: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                                let mut v157: i32 = 0i32;
                                                let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let mut v159: Rc<str> = inline_from_18(v17.clone(), v157, v158.clone());
                                                let mut v160: Rc<str> = Rc::<str>::from(format!("{}{}", v156.clone(), v159.clone()));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v155.clone(), v160.clone());
                                                continue;
                                            }
                                            _ => {
                                                let mut v162: US0 = US0::US0_MdParagraph;
                                                let mut v166: Rc<str> = match &v2 {
                                                    US0::US0_MdList => {
                                                        let mut v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                                        v164.clone()
                                                    }
                                                    US0::US0_MdNone => {
                                                        let mut v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v163.clone()
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                let mut v167: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v166.clone()));
                                                let mut v168: Rc<str> = Rc::<str>::from(format!("{}{}", v167.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) }));
                                                let mut v169: i32 = 0i32;
                                                let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let mut v171: Rc<str> = inline_from_18(v17.clone(), v169, v170.clone());
                                                let mut v172: Rc<str> = Rc::<str>::from(format!("{}{}", v168.clone(), v171.clone()));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v162.clone(), v172.clone());
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn flag_at_20(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v1 == v2;
        if v3 {
            return -1i32;
        } else {
            let mut v5: Rc<str> = Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
            let mut v6: bool = v5.clone() == v0.clone();
            if v6 {
                return v1;
            } else {
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v7, v2);
                continue;
            }
        }
    }
}
fn spiral_main() -> i32 {
    let mut v1: i32 = std::env::args().count() as i32;
    let mut v2: bool = v1 == 2i32;
    if v2 {
        let mut v4: Rc<str> = Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
        let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--self-test"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v7: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let mut v9: i64 = std::process::id() as i64;
            let mut v10: Rc<str> = show_i64_0(v9);
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
            let mut v13: i64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64;
            let mut v14: Rc<str> = show_i64_0(v13);
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), v14.clone()));
            let mut v17: Rc<str> = Rc::<str>::from(std::path::Path::new(v7.as_ref()).join(v15.as_ref()).display().to_string());
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_.root"); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = Rc::<str>::from(std::path::Path::new(v17.as_ref()).join(v19.as_ref()).display().to_string());
            let mut v21: i64 = 3i64;
            let mut v22: i32 = lay_1(v20.clone(), v21);
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = { let mut out = String::new(); if let Ok(read) = std::fs::read_dir(v17.as_ref()) { for entry in read.flatten() { let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue }; let child = entry.path(); let Some(name) = child.file_name() else { continue }; let name = name.to_string_lossy(); if child.is_dir() { out.push(char::from(49)); } else if child.is_file() { out.push(char::from(48)); } else { continue }; out.push(char::from(32)); out.push_str(&name.len().to_string()); out.push(char::from(10)); out.push_str(&name); } } Rc::<str>::from(out) };
            let mut v26: i32 = 0i32;
            let mut v27: Rc<UH0> = parse_4(v25.clone(), v26);
            let mut v28: Rc<UH0> = sort_7(v27.clone());
            let mut v29: Rc<UH1> = chain_10(v17.clone(), v23.clone(), v28.clone());
            let mut v30: Rc<str> = html_chain_11(v29.clone());
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) }, v30.clone()));
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) }));
            let mut v33: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div><details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details></div>\n</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
            let mut v62: i32 = if v33 {
                0i32
            } else {
                let mut v35: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-got.html"); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = Rc::<str>::from(std::path::Path::new(v35.as_ref()).join(v37.as_ref()).display().to_string());
                let mut v39: i32 = 0i32;
                let mut v40: i32 = -1i32;
                let mut v41: i32 = last_sep_3(v38.clone(), v39, v40);
                let mut v42: bool = v41 == -1i32;
                let mut v52: Rc<str> = if v42 {
                    v23.clone()
                } else {
                    let mut v43: i32 = v41.wrapping_sub(1i32);
                    let mut v44: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v43 as i64);
                    let mut v45: i32 = (v44.clone().len() as i32);
                    let mut v46: bool = v45 == 2i32;
                    let mut v49: bool = if v46 {
                        let mut v47: u8 = v44.clone().as_bytes()[1i32 as usize];
                        let mut v48: bool = v47 == b':';
                        v48
                    } else {
                        false
                    };
                    if v49 {
                        let mut v50: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v41 as i64);
                        v50.clone()
                    } else {
                        v44.clone()
                    }
                };
                let mut v53: i32 = (v52.clone().len() as i32);
                let mut v54: bool = v53 == 0i32;
                let mut v57: i32 = if v54 {
                    0i32
                } else {
                    let mut v56: i32 = i32::from(std::fs::create_dir_all(v52.as_ref()).is_err());
                    v56
                };
                let mut v58: bool = v57 == 0i32;
                let mut v61: i32 = if v58 {
                    let mut v60: i32 = i32::from(std::fs::write(v38.as_ref(), v32.as_ref().as_bytes()).is_err());
                    v60
                } else {
                    v57
                };
                1i32
            };
            let mut v63: i64 = folder_bytes_12(v29.clone());
            let mut v64: bool = v63 == 10i64;
            let mut v65: i32 = if v64 {
                0i32
            } else {
                3i32
            };
            let mut v67: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let mut v69: i64 = std::process::id() as i64;
            let mut v70: Rc<str> = show_i64_0(v69);
            let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
            let mut v73: i64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64;
            let mut v74: Rc<str> = show_i64_0(v73);
            let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v71.clone(), v74.clone()));
            let mut v77: Rc<str> = Rc::<str>::from(std::path::Path::new(v67.as_ref()).join(v75.as_ref()).display().to_string());
            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v80: Rc<str> = Rc::<str>::from(std::path::Path::new(v77.as_ref()).join(v79.as_ref()).display().to_string());
            let mut v82: i32 = i32::from(std::fs::create_dir_all(v80.as_ref()).is_err());
            let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.txt"); } LIT.with(|lit| lit.clone()) };
            let mut v85: Rc<str> = Rc::<str>::from(std::path::Path::new(v77.as_ref()).join(v84.as_ref()).display().to_string());
            let mut v86: i32 = 0i32;
            let mut v87: i32 = -1i32;
            let mut v88: i32 = last_sep_3(v85.clone(), v86, v87);
            let mut v89: bool = v88 == -1i32;
            let mut v99: Rc<str> = if v89 {
                v23.clone()
            } else {
                let mut v90: i32 = v88.wrapping_sub(1i32);
                let mut v91: Rc<str> = string_slice(&v85.clone(), 0i32 as i64, v90 as i64);
                let mut v92: i32 = (v91.clone().len() as i32);
                let mut v93: bool = v92 == 2i32;
                let mut v96: bool = if v93 {
                    let mut v94: u8 = v91.clone().as_bytes()[1i32 as usize];
                    let mut v95: bool = v94 == b':';
                    v95
                } else {
                    false
                };
                if v96 {
                    let mut v97: Rc<str> = string_slice(&v85.clone(), 0i32 as i64, v88 as i64);
                    v97.clone()
                } else {
                    v91.clone()
                }
            };
            let mut v100: i32 = (v99.clone().len() as i32);
            let mut v101: bool = v100 == 0i32;
            let mut v104: i32 = if v101 {
                0i32
            } else {
                let mut v103: i32 = i32::from(std::fs::create_dir_all(v99.as_ref()).is_err());
                v103
            };
            let mut v105: bool = v104 == 0i32;
            let mut v109: i32 = if v105 {
                let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x"); } LIT.with(|lit| lit.clone()) };
                let mut v108: i32 = i32::from(std::fs::write(v85.as_ref(), v107.as_ref().as_bytes()).is_err());
                v108
            } else {
                v104
            };
            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.txt"); } LIT.with(|lit| lit.clone()) };
            let mut v112: Rc<str> = Rc::<str>::from(std::path::Path::new(v77.as_ref()).join(v111.as_ref()).display().to_string());
            let mut v113: i32 = 0i32;
            let mut v114: i32 = -1i32;
            let mut v115: i32 = last_sep_3(v112.clone(), v113, v114);
            let mut v116: bool = v115 == -1i32;
            let mut v126: Rc<str> = if v116 {
                v23.clone()
            } else {
                let mut v117: i32 = v115.wrapping_sub(1i32);
                let mut v118: Rc<str> = string_slice(&v112.clone(), 0i32 as i64, v117 as i64);
                let mut v119: i32 = (v118.clone().len() as i32);
                let mut v120: bool = v119 == 2i32;
                let mut v123: bool = if v120 {
                    let mut v121: u8 = v118.clone().as_bytes()[1i32 as usize];
                    let mut v122: bool = v121 == b':';
                    v122
                } else {
                    false
                };
                if v123 {
                    let mut v124: Rc<str> = string_slice(&v112.clone(), 0i32 as i64, v115 as i64);
                    v124.clone()
                } else {
                    v118.clone()
                }
            };
            let mut v127: i32 = (v126.clone().len() as i32);
            let mut v128: bool = v127 == 0i32;
            let mut v131: i32 = if v128 {
                0i32
            } else {
                let mut v130: i32 = i32::from(std::fs::create_dir_all(v126.as_ref()).is_err());
                v130
            };
            let mut v132: bool = v131 == 0i32;
            let mut v136: i32 = if v132 {
                let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yy"); } LIT.with(|lit| lit.clone()) };
                let mut v135: i32 = i32::from(std::fs::write(v112.as_ref(), v134.as_ref().as_bytes()).is_err());
                v135
            } else {
                v131
            };
            let mut v138: Rc<str> = Rc::<str>::from(std::path::Path::new(v77.as_ref()).join(v79.as_ref()).display().to_string());
            let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let mut v141: Rc<str> = Rc::<str>::from(std::path::Path::new(v138.as_ref()).join(v140.as_ref()).display().to_string());
            let mut v142: i32 = 0i32;
            let mut v143: i32 = -1i32;
            let mut v144: i32 = last_sep_3(v141.clone(), v142, v143);
            let mut v145: bool = v144 == -1i32;
            let mut v155: Rc<str> = if v145 {
                v23.clone()
            } else {
                let mut v146: i32 = v144.wrapping_sub(1i32);
                let mut v147: Rc<str> = string_slice(&v141.clone(), 0i32 as i64, v146 as i64);
                let mut v148: i32 = (v147.clone().len() as i32);
                let mut v149: bool = v148 == 2i32;
                let mut v152: bool = if v149 {
                    let mut v150: u8 = v147.clone().as_bytes()[1i32 as usize];
                    let mut v151: bool = v150 == b':';
                    v151
                } else {
                    false
                };
                if v152 {
                    let mut v153: Rc<str> = string_slice(&v141.clone(), 0i32 as i64, v144 as i64);
                    v153.clone()
                } else {
                    v147.clone()
                }
            };
            let mut v156: i32 = (v155.clone().len() as i32);
            let mut v157: bool = v156 == 0i32;
            let mut v160: i32 = if v157 {
                0i32
            } else {
                let mut v159: i32 = i32::from(std::fs::create_dir_all(v155.as_ref()).is_err());
                v159
            };
            let mut v161: bool = v160 == 0i32;
            let mut v165: i32 = if v161 {
                let mut v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("z"); } LIT.with(|lit| lit.clone()) };
                let mut v164: i32 = i32::from(std::fs::write(v141.as_ref(), v163.as_ref().as_bytes()).is_err());
                v164
            } else {
                v160
            };
            let mut v167: Rc<str> = { let mut out = String::new(); if let Ok(read) = std::fs::read_dir(v77.as_ref()) { for entry in read.flatten() { let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue }; let child = entry.path(); let Some(name) = child.file_name() else { continue }; let name = name.to_string_lossy(); if child.is_dir() { out.push(char::from(49)); } else if child.is_file() { out.push(char::from(48)); } else { continue }; out.push(char::from(32)); out.push_str(&name.len().to_string()); out.push(char::from(10)); out.push_str(&name); } } Rc::<str>::from(out) };
            let mut v168: i32 = 0i32;
            let mut v169: Rc<UH0> = parse_4(v167.clone(), v168);
            let mut v170: Rc<UH0> = sort_7(v169.clone());
            let mut v171: Rc<UH1> = chain_10(v77.clone(), v23.clone(), v170.clone());
            let mut v172: Rc<str> = html_chain_11(v171.clone());
            let mut v173: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) }, v172.clone()));
            let mut v174: Rc<str> = Rc::<str>::from(format!("{}{}", v173.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) }));
            let mut v175: bool = v174.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div><details open=\"true\"><summary>&#128194; <a href=\"m\">m</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"m/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"a.txt\">a.txt</a><span> (2.00 B)</span></div><div>&#128196; <a href=\"b.txt\">b.txt</a><span> (1.00 B)</span></div></div>\n</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
            let mut v204: i32 = if v175 {
                0i32
            } else {
                let mut v177: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
                let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-order-got.html"); } LIT.with(|lit| lit.clone()) };
                let mut v180: Rc<str> = Rc::<str>::from(std::path::Path::new(v177.as_ref()).join(v179.as_ref()).display().to_string());
                let mut v181: i32 = 0i32;
                let mut v182: i32 = -1i32;
                let mut v183: i32 = last_sep_3(v180.clone(), v181, v182);
                let mut v184: bool = v183 == -1i32;
                let mut v194: Rc<str> = if v184 {
                    v23.clone()
                } else {
                    let mut v185: i32 = v183.wrapping_sub(1i32);
                    let mut v186: Rc<str> = string_slice(&v180.clone(), 0i32 as i64, v185 as i64);
                    let mut v187: i32 = (v186.clone().len() as i32);
                    let mut v188: bool = v187 == 2i32;
                    let mut v191: bool = if v188 {
                        let mut v189: u8 = v186.clone().as_bytes()[1i32 as usize];
                        let mut v190: bool = v189 == b':';
                        v190
                    } else {
                        false
                    };
                    if v191 {
                        let mut v192: Rc<str> = string_slice(&v180.clone(), 0i32 as i64, v183 as i64);
                        v192.clone()
                    } else {
                        v186.clone()
                    }
                };
                let mut v195: i32 = (v194.clone().len() as i32);
                let mut v196: bool = v195 == 0i32;
                let mut v199: i32 = if v196 {
                    0i32
                } else {
                    let mut v198: i32 = i32::from(std::fs::create_dir_all(v194.as_ref()).is_err());
                    v198
                };
                let mut v200: bool = v199 == 0i32;
                let mut v203: i32 = if v200 {
                    let mut v202: i32 = i32::from(std::fs::write(v180.as_ref(), v174.as_ref().as_bytes()).is_err());
                    v202
                } else {
                    v199
                };
                1i32
            };
            let mut v206: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
            let mut v208: i64 = std::process::id() as i64;
            let mut v209: Rc<str> = show_i64_0(v208);
            let mut v210: Rc<str> = Rc::<str>::from(format!("{}{}", v209.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
            let mut v212: i64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64;
            let mut v213: Rc<str> = show_i64_0(v212);
            let mut v214: Rc<str> = Rc::<str>::from(format!("{}{}", v210.clone(), v213.clone()));
            let mut v216: Rc<str> = Rc::<str>::from(std::path::Path::new(v206.as_ref()).join(v214.as_ref()).display().to_string());
            let mut v218: i32 = i32::from(std::fs::create_dir_all(v216.as_ref()).is_err());
            let mut v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
            let mut v221: Rc<str> = Rc::<str>::from(std::path::Path::new(v216.as_ref()).join(v220.as_ref()).display().to_string());
            let mut v222: i32 = 0i32;
            let mut v223: i32 = -1i32;
            let mut v224: i32 = last_sep_3(v221.clone(), v222, v223);
            let mut v225: bool = v224 == -1i32;
            let mut v235: Rc<str> = if v225 {
                v23.clone()
            } else {
                let mut v226: i32 = v224.wrapping_sub(1i32);
                let mut v227: Rc<str> = string_slice(&v221.clone(), 0i32 as i64, v226 as i64);
                let mut v228: i32 = (v227.clone().len() as i32);
                let mut v229: bool = v228 == 2i32;
                let mut v232: bool = if v229 {
                    let mut v230: u8 = v227.clone().as_bytes()[1i32 as usize];
                    let mut v231: bool = v230 == b':';
                    v231
                } else {
                    false
                };
                if v232 {
                    let mut v233: Rc<str> = string_slice(&v221.clone(), 0i32 as i64, v224 as i64);
                    v233.clone()
                } else {
                    v227.clone()
                }
            };
            let mut v236: i32 = (v235.clone().len() as i32);
            let mut v237: bool = v236 == 0i32;
            let mut v240: i32 = if v237 {
                0i32
            } else {
                let mut v239: i32 = i32::from(std::fs::create_dir_all(v235.as_ref()).is_err());
                v239
            };
            let mut v241: bool = v240 == 0i32;
            let mut v245: i32 = if v241 {
                let mut v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# Title\n\nSome `code` and [a link](x.html).\n\n- one\n- two <b>\n\n```\nlet x = 1 < 2\n```\n"); } LIT.with(|lit| lit.clone()) };
                let mut v244: i32 = i32::from(std::fs::write(v221.as_ref(), v243.as_ref().as_bytes()).is_err());
                v244
            } else {
                v240
            };
            let mut v247: Rc<str> = Rc::<str>::from(std::path::Path::new(v216.as_ref()).join(v220.as_ref()).display().to_string());
            let mut v249: bool = std::path::Path::new(v247.as_ref()).is_file();
            let mut v257: Rc<str> = if v249 {
                let mut v251: Rc<str> = Rc::<str>::from(std::fs::read_to_string(v247.as_ref()).map(|text| text.lines().collect::<Vec<_>>().join(&char::from(10).to_string())).unwrap_or_default());
                let mut v252: i32 = 0i32;
                let mut v253: US0 = US0::US0_MdNone;
                let mut v254: Rc<str> = markdown_lines_13(v251.clone(), v252, v253.clone(), v23.clone());
                let mut v255: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) }, v254.clone()));
                let mut v256: Rc<str> = Rc::<str>::from(format!("{}{}", v255.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) }));
                v256.clone()
            } else {
                v23.clone()
            };
            let mut v258: bool = v257.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n<h1>Title</h1>\n<p>Some <code>code</code> and <a href=\"x.html\">a link</a>.</p>\n<ul>\n<li>one</li>\n<li>two &lt;b&gt;</li>\n</ul>\n<pre><code>let x = 1 &lt; 2\n</code></pre>\n</section>\n"); } LIT.with(|lit| lit.clone()) };
            let mut v287: i32 = if v258 {
                0i32
            } else {
                let mut v260: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
                let mut v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-readme-got.html"); } LIT.with(|lit| lit.clone()) };
                let mut v263: Rc<str> = Rc::<str>::from(std::path::Path::new(v260.as_ref()).join(v262.as_ref()).display().to_string());
                let mut v264: i32 = 0i32;
                let mut v265: i32 = -1i32;
                let mut v266: i32 = last_sep_3(v263.clone(), v264, v265);
                let mut v267: bool = v266 == -1i32;
                let mut v277: Rc<str> = if v267 {
                    v23.clone()
                } else {
                    let mut v268: i32 = v266.wrapping_sub(1i32);
                    let mut v269: Rc<str> = string_slice(&v263.clone(), 0i32 as i64, v268 as i64);
                    let mut v270: i32 = (v269.clone().len() as i32);
                    let mut v271: bool = v270 == 2i32;
                    let mut v274: bool = if v271 {
                        let mut v272: u8 = v269.clone().as_bytes()[1i32 as usize];
                        let mut v273: bool = v272 == b':';
                        v273
                    } else {
                        false
                    };
                    if v274 {
                        let mut v275: Rc<str> = string_slice(&v263.clone(), 0i32 as i64, v266 as i64);
                        v275.clone()
                    } else {
                        v269.clone()
                    }
                };
                let mut v278: i32 = (v277.clone().len() as i32);
                let mut v279: bool = v278 == 0i32;
                let mut v282: i32 = if v279 {
                    0i32
                } else {
                    let mut v281: i32 = i32::from(std::fs::create_dir_all(v277.as_ref()).is_err());
                    v281
                };
                let mut v283: bool = v282 == 0i32;
                let mut v286: i32 = if v283 {
                    let mut v285: i32 = i32::from(std::fs::write(v263.as_ref(), v257.as_ref().as_bytes()).is_err());
                    v285
                } else {
                    v282
                };
                1i32
            };
            let mut v288: i64 = 1024i64;
            let mut v289: Rc<str> = show_i64_0(v288);
            let mut v290: Rc<str> = Rc::<str>::from(format!("{}{}", v289.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
            let mut v291: i64 = 0i64;
            let mut v292: Rc<str> = show_i64_0(v291);
            let mut v293: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v292.clone()));
            let mut v294: Rc<str> = Rc::<str>::from(format!("{}{}", v290.clone(), v293.clone()));
            let mut v295: Rc<str> = Rc::<str>::from(format!("{}{}", v294.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) }));
            let mut v296: bool = v295.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1024.00 B"); } LIT.with(|lit| lit.clone()) };
            let mut v327: i32 = if v296 {
                let mut v297: i64 = 1i64;
                let mut v298: Rc<str> = show_i64_0(v297);
                let mut v299: Rc<str> = Rc::<str>::from(format!("{}{}", v298.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                let mut v300: i64 = 0i64;
                let mut v301: Rc<str> = show_i64_0(v300);
                let mut v302: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v301.clone()));
                let mut v303: Rc<str> = Rc::<str>::from(format!("{}{}", v299.clone(), v302.clone()));
                let mut v304: Rc<str> = Rc::<str>::from(format!("{}{}", v303.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) }));
                let mut v305: bool = v304.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1.00 KB"); } LIT.with(|lit| lit.clone()) };
                if v305 {
                    let mut v306: i64 = 1024i64;
                    let mut v307: Rc<str> = show_i64_0(v306);
                    let mut v308: Rc<str> = Rc::<str>::from(format!("{}{}", v307.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                    let mut v309: i64 = 0i64;
                    let mut v310: Rc<str> = show_i64_0(v309);
                    let mut v311: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v310.clone()));
                    let mut v312: Rc<str> = Rc::<str>::from(format!("{}{}", v308.clone(), v311.clone()));
                    let mut v313: Rc<str> = Rc::<str>::from(format!("{}{}", v312.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) }));
                    let mut v314: bool = v313.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1024.00 KB"); } LIT.with(|lit| lit.clone()) };
                    if v314 {
                        let mut v315: i64 = 1i64;
                        let mut v316: Rc<str> = show_i64_0(v315);
                        let mut v317: Rc<str> = Rc::<str>::from(format!("{}{}", v316.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
                        let mut v318: i64 = 0i64;
                        let mut v319: Rc<str> = show_i64_0(v318);
                        let mut v320: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v319.clone()));
                        let mut v321: Rc<str> = Rc::<str>::from(format!("{}{}", v317.clone(), v320.clone()));
                        let mut v322: Rc<str> = Rc::<str>::from(format!("{}{}", v321.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) }));
                        let mut v323: bool = v322.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1.00 MB"); } LIT.with(|lit| lit.clone()) };
                        if v323 {
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
            let mut v328: bool = v62 == 0i32;
            if v328 {
                let mut v329: bool = v65 == 0i32;
                if v329 {
                    let mut v330: bool = v204 == 0i32;
                    if v330 {
                        let mut v331: bool = v287 == 0i32;
                        if v331 {
                            v327
                        } else {
                            4i32
                        }
                    } else {
                        v204
                    }
                } else {
                    v65
                }
            } else {
                v62
            }
        } else {
            let mut v336: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
            let mut v337: i32 = 1i32;
            let mut v339: i32 = std::env::args().count() as i32;
            let mut v340: i32 = flag_at_20(v336.clone(), v337, v339);
            let mut v341: bool = v340 == -1i32;
            let mut v347: i32 = if v341 {
                -1i32
            } else {
                let mut v342: i32 = v340.wrapping_add(1i32);
                let mut v344: i32 = std::env::args().count() as i32;
                let mut v345: bool = v342 < v344;
                if v345 {
                    v342
                } else {
                    -1i32
                }
            };
            let mut v348: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
            let mut v349: i32 = 1i32;
            let mut v351: i32 = std::env::args().count() as i32;
            let mut v352: i32 = flag_at_20(v348.clone(), v349, v351);
            let mut v353: bool = v352 == -1i32;
            let mut v359: i32 = if v353 {
                -1i32
            } else {
                let mut v354: i32 = v352.wrapping_add(1i32);
                let mut v356: i32 = std::env::args().count() as i32;
                let mut v357: bool = v354 < v356;
                if v357 {
                    v354
                } else {
                    -1i32
                }
            };
            let mut v360: bool = v347 == -1i32;
            if v360 {
                2i32
            } else {
                let mut v361: bool = v359 == -1i32;
                if v361 {
                    2i32
                } else {
                    let mut v363: Rc<str> = Rc::<str>::from(std::env::args().nth(v359 as usize).unwrap_or_default());
                    let mut v365: Rc<str> = Rc::<str>::from(std::env::args().nth(v347 as usize).unwrap_or_default());
                    let mut v366: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v368: Rc<str> = { let mut out = String::new(); if let Ok(read) = std::fs::read_dir(v365.as_ref()) { for entry in read.flatten() { let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue }; let child = entry.path(); let Some(name) = child.file_name() else { continue }; let name = name.to_string_lossy(); if child.is_dir() { out.push(char::from(49)); } else if child.is_file() { out.push(char::from(48)); } else { continue }; out.push(char::from(32)); out.push_str(&name.len().to_string()); out.push(char::from(10)); out.push_str(&name); } } Rc::<str>::from(out) };
                    let mut v369: i32 = 0i32;
                    let mut v370: Rc<UH0> = parse_4(v368.clone(), v369);
                    let mut v371: Rc<UH0> = sort_7(v370.clone());
                    let mut v372: Rc<UH1> = chain_10(v365.clone(), v366.clone(), v371.clone());
                    let mut v374: Rc<str> = Rc::<str>::from(std::env::args().nth(v347 as usize).unwrap_or_default());
                    let mut v376: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
                    let mut v377: Rc<str> = Rc::<str>::from(std::path::Path::new(v374.as_ref()).join(v376.as_ref()).display().to_string());
                    let mut v379: bool = std::path::Path::new(v377.as_ref()).is_file();
                    let mut v387: Rc<str> = if v379 {
                        let mut v381: Rc<str> = Rc::<str>::from(std::fs::read_to_string(v377.as_ref()).map(|text| text.lines().collect::<Vec<_>>().join(&char::from(10).to_string())).unwrap_or_default());
                        let mut v382: i32 = 0i32;
                        let mut v383: US0 = US0::US0_MdNone;
                        let mut v384: Rc<str> = markdown_lines_13(v381.clone(), v382, v383.clone(), v366.clone());
                        let mut v385: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) }, v384.clone()));
                        let mut v386: Rc<str> = Rc::<str>::from(format!("{}{}", v385.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) }));
                        v386.clone()
                    } else {
                        v366.clone()
                    };
                    let mut v388: Rc<str> = html_chain_11(v372.clone());
                    let mut v389: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) }, v388.clone()));
                    let mut v390: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) }, v387.clone()));
                    let mut v391: Rc<str> = Rc::<str>::from(format!("{}{}", v390.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) }));
                    let mut v392: Rc<str> = Rc::<str>::from(format!("{}{}", v389.clone(), v391.clone()));
                    let mut v393: i32 = 0i32;
                    let mut v394: i32 = -1i32;
                    let mut v395: i32 = last_sep_3(v363.clone(), v393, v394);
                    let mut v396: bool = v395 == -1i32;
                    let mut v406: Rc<str> = if v396 {
                        v366.clone()
                    } else {
                        let mut v397: i32 = v395.wrapping_sub(1i32);
                        let mut v398: Rc<str> = string_slice(&v363.clone(), 0i32 as i64, v397 as i64);
                        let mut v399: i32 = (v398.clone().len() as i32);
                        let mut v400: bool = v399 == 2i32;
                        let mut v403: bool = if v400 {
                            let mut v401: u8 = v398.clone().as_bytes()[1i32 as usize];
                            let mut v402: bool = v401 == b':';
                            v402
                        } else {
                            false
                        };
                        if v403 {
                            let mut v404: Rc<str> = string_slice(&v363.clone(), 0i32 as i64, v395 as i64);
                            v404.clone()
                        } else {
                            v398.clone()
                        }
                    };
                    let mut v407: i32 = (v406.clone().len() as i32);
                    let mut v408: bool = v407 == 0i32;
                    let mut v411: i32 = if v408 {
                        0i32
                    } else {
                        let mut v410: i32 = i32::from(std::fs::create_dir_all(v406.as_ref()).is_err());
                        v410
                    };
                    let mut v412: bool = v411 == 0i32;
                    if v412 {
                        let mut v414: i32 = i32::from(std::fs::write(v363.as_ref(), v392.as_ref().as_bytes()).is_err());
                        v414
                    } else {
                        v411
                    }
                }
            }
        }
    } else {
        let mut v419: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
        let mut v420: i32 = 1i32;
        let mut v422: i32 = std::env::args().count() as i32;
        let mut v423: i32 = flag_at_20(v419.clone(), v420, v422);
        let mut v424: bool = v423 == -1i32;
        let mut v430: i32 = if v424 {
            -1i32
        } else {
            let mut v425: i32 = v423.wrapping_add(1i32);
            let mut v427: i32 = std::env::args().count() as i32;
            let mut v428: bool = v425 < v427;
            if v428 {
                v425
            } else {
                -1i32
            }
        };
        let mut v431: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
        let mut v432: i32 = 1i32;
        let mut v434: i32 = std::env::args().count() as i32;
        let mut v435: i32 = flag_at_20(v431.clone(), v432, v434);
        let mut v436: bool = v435 == -1i32;
        let mut v442: i32 = if v436 {
            -1i32
        } else {
            let mut v437: i32 = v435.wrapping_add(1i32);
            let mut v439: i32 = std::env::args().count() as i32;
            let mut v440: bool = v437 < v439;
            if v440 {
                v437
            } else {
                -1i32
            }
        };
        let mut v443: bool = v430 == -1i32;
        if v443 {
            2i32
        } else {
            let mut v444: bool = v442 == -1i32;
            if v444 {
                2i32
            } else {
                let mut v446: Rc<str> = Rc::<str>::from(std::env::args().nth(v442 as usize).unwrap_or_default());
                let mut v448: Rc<str> = Rc::<str>::from(std::env::args().nth(v430 as usize).unwrap_or_default());
                let mut v449: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v451: Rc<str> = { let mut out = String::new(); if let Ok(read) = std::fs::read_dir(v448.as_ref()) { for entry in read.flatten() { let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue }; let child = entry.path(); let Some(name) = child.file_name() else { continue }; let name = name.to_string_lossy(); if child.is_dir() { out.push(char::from(49)); } else if child.is_file() { out.push(char::from(48)); } else { continue }; out.push(char::from(32)); out.push_str(&name.len().to_string()); out.push(char::from(10)); out.push_str(&name); } } Rc::<str>::from(out) };
                let mut v452: i32 = 0i32;
                let mut v453: Rc<UH0> = parse_4(v451.clone(), v452);
                let mut v454: Rc<UH0> = sort_7(v453.clone());
                let mut v455: Rc<UH1> = chain_10(v448.clone(), v449.clone(), v454.clone());
                let mut v457: Rc<str> = Rc::<str>::from(std::env::args().nth(v430 as usize).unwrap_or_default());
                let mut v459: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
                let mut v460: Rc<str> = Rc::<str>::from(std::path::Path::new(v457.as_ref()).join(v459.as_ref()).display().to_string());
                let mut v462: bool = std::path::Path::new(v460.as_ref()).is_file();
                let mut v470: Rc<str> = if v462 {
                    let mut v464: Rc<str> = Rc::<str>::from(std::fs::read_to_string(v460.as_ref()).map(|text| text.lines().collect::<Vec<_>>().join(&char::from(10).to_string())).unwrap_or_default());
                    let mut v465: i32 = 0i32;
                    let mut v466: US0 = US0::US0_MdNone;
                    let mut v467: Rc<str> = markdown_lines_13(v464.clone(), v465, v466.clone(), v449.clone());
                    let mut v468: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) }, v467.clone()));
                    let mut v469: Rc<str> = Rc::<str>::from(format!("{}{}", v468.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) }));
                    v469.clone()
                } else {
                    v449.clone()
                };
                let mut v471: Rc<str> = html_chain_11(v455.clone());
                let mut v472: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) }, v471.clone()));
                let mut v473: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) }, v470.clone()));
                let mut v474: Rc<str> = Rc::<str>::from(format!("{}{}", v473.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) }));
                let mut v475: Rc<str> = Rc::<str>::from(format!("{}{}", v472.clone(), v474.clone()));
                let mut v476: i32 = 0i32;
                let mut v477: i32 = -1i32;
                let mut v478: i32 = last_sep_3(v446.clone(), v476, v477);
                let mut v479: bool = v478 == -1i32;
                let mut v489: Rc<str> = if v479 {
                    v449.clone()
                } else {
                    let mut v480: i32 = v478.wrapping_sub(1i32);
                    let mut v481: Rc<str> = string_slice(&v446.clone(), 0i32 as i64, v480 as i64);
                    let mut v482: i32 = (v481.clone().len() as i32);
                    let mut v483: bool = v482 == 2i32;
                    let mut v486: bool = if v483 {
                        let mut v484: u8 = v481.clone().as_bytes()[1i32 as usize];
                        let mut v485: bool = v484 == b':';
                        v485
                    } else {
                        false
                    };
                    if v486 {
                        let mut v487: Rc<str> = string_slice(&v446.clone(), 0i32 as i64, v478 as i64);
                        v487.clone()
                    } else {
                        v481.clone()
                    }
                };
                let mut v490: i32 = (v489.clone().len() as i32);
                let mut v491: bool = v490 == 0i32;
                let mut v494: i32 = if v491 {
                    0i32
                } else {
                    let mut v493: i32 = i32::from(std::fs::create_dir_all(v489.as_ref()).is_err());
                    v493
                };
                let mut v495: bool = v494 == 0i32;
                if v495 {
                    let mut v497: i32 = i32::from(std::fs::write(v446.as_ref(), v475.as_ref().as_bytes()).is_err());
                    v497
                } else {
                    v494
                }
            }
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
