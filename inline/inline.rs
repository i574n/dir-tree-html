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
fn escape_from_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 == v4;
        if v5 {
            let v6: bool = v2 == v1;
            let v10: Rc<str> = if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7.clone()
            } else {
                let v8: i32 = v1.wrapping_sub(1i32);
                let v9: Rc<str> = string_slice(&v0.clone(), v2 as i64, v8 as i64);
                v9.clone()
            };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v10));
            return v11.clone();
        } else {
            let v12: u8 = v0.clone().as_bytes()[v1 as usize];
            let v13: bool = v12 == b'&';
            let v25: Rc<str> = if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&amp;"); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                let v15: bool = v12 == b'<';
                if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&lt;"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let v17: bool = v12 == b'>';
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&gt;"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let v19: bool = v12 == b'"';
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&quot;"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        }
                    }
                }
            };
            let v26: bool = &*v25 == "";
            if v26 {
                let v27: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v27, v2, v3.clone());
                continue;
            } else {
                let v29: i32 = v1.wrapping_add(1i32);
                let v30: bool = v2 == v1;
                let v34: Rc<str> = if v30 {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v31.clone()
                } else {
                    let v32: i32 = v1.wrapping_sub(1i32);
                    let v33: Rc<str> = string_slice(&v0.clone(), v2 as i64, v32 as i64);
                    v33.clone()
                };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v34)), v25));
                (v0, v1, v2, v3) = (v0.clone(), v29, v29, v36.clone());
                continue;
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: i32 = 0i32;
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        escape_from_0(v0.clone(), v1, v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn find_char_from_2(mut v0: Rc<str>, mut v1: u8, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v2 == v3;
        if v4 {
            return -1i32;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v2 as usize];
            let v6: bool = v5 == v1;
            if v6 {
                return v2;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v7);
                continue;
            }
        }
    }
}
fn next_inline_marker_from_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'`';
            if v5 {
                return v1;
            } else {
                let v6: bool = v4 == b'[';
                if v6 {
                    return v1;
                } else {
                    let v7: bool = v4 == b'!';
                    if v7 {
                        return v1;
                    } else {
                        let v8: bool = v4 == b'*';
                        if v8 {
                            return v1;
                        } else {
                            let v9: bool = v4 == b'<';
                            if v9 {
                                return v1;
                            } else {
                                let v10: bool = v4 == b'\\';
                                if v10 {
                                    return v1;
                                } else {
                                    let v11: i32 = v1.wrapping_add(1i32);
                                    (v0, v1) = (v0.clone(), v11);
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
fn double_close_4(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return -1i32;
        } else {
            let v4: i32 = v1.wrapping_add(1i32);
            let v5: bool = v4 == v2;
            if v5 {
                return -1i32;
            } else {
                let v6: u8 = v0.clone().as_bytes()[v1 as usize];
                let v7: bool = v6 == b'*';
                if v7 {
                    let v8: u8 = v0.clone().as_bytes()[v4 as usize];
                    let v9: bool = v8 == b'*';
                    if v9 {
                        return v1;
                    } else {
                        (v0, v1) = (v0.clone(), v4);
                        continue;
                    }
                } else {
                    (v0, v1) = (v0.clone(), v4);
                    continue;
                }
            }
        }
    }
}
fn image_close_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: u8 = v0.clone().as_bytes()[v1 as usize];
    let v4: bool = v3 == b'!';
    if v4 {
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: bool = v5 == v2;
        if v6 {
            -1i32
        } else {
            let v7: u8 = v0.clone().as_bytes()[v5 as usize];
            let v8: bool = v7 == b'[';
            if v8 {
                let v9: u8 = b']';
                let v10: i32 = v1.wrapping_add(2i32);
                let v11: i32 = find_char_from_2(v0.clone(), v9, v10);
                let v12: bool = v11 == -1i32;
                if v12 {
                    -1i32
                } else {
                    let v13: i32 = v11.wrapping_add(1i32);
                    let v14: bool = v13 == v2;
                    if v14 {
                        -1i32
                    } else {
                        let v15: u8 = v0.clone().as_bytes()[v13 as usize];
                        let v16: bool = v15 == b'(';
                        if v16 {
                            let v17: u8 = b')';
                            let v18: i32 = v11.wrapping_add(2i32);
                            find_char_from_2(v0, v17, v18)
                        } else {
                            -1i32
                        }
                    }
                }
            } else {
                -1i32
            }
        }
    } else {
        -1i32
    }
}
fn badge_close_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: u8 = v0.clone().as_bytes()[v1 as usize];
    let v4: bool = v3 == b'[';
    if v4 {
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: bool = v5 == v2;
        if v6 {
            -1i32
        } else {
            let v7: i32 = image_close_5(v0.clone(), v5);
            let v8: bool = v7 == -1i32;
            if v8 {
                -1i32
            } else {
                let v9: i32 = v7.wrapping_add(1i32);
                let v10: bool = v9 == v2;
                if v10 {
                    -1i32
                } else {
                    let v11: u8 = v0.clone().as_bytes()[v9 as usize];
                    let v12: bool = v11 == b']';
                    if v12 {
                        let v13: i32 = v7.wrapping_add(2i32);
                        let v14: bool = v13 == v2;
                        if v14 {
                            -1i32
                        } else {
                            let v15: u8 = v0.clone().as_bytes()[v13 as usize];
                            let v16: bool = v15 == b'(';
                            if v16 {
                                let v17: u8 = b')';
                                let v18: i32 = v7.wrapping_add(3i32);
                                find_char_from_2(v0, v17, v18)
                            } else {
                                -1i32
                            }
                        }
                    } else {
                        -1i32
                    }
                }
            }
        }
    } else {
        -1i32
    }
}
fn inline_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 == v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'`';
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                let v8: i32 = find_char_from_2(v0.clone(), v5, v7);
                let v9: bool = v8 == -1i32;
                if v9 {
                    let v10: i32 = next_inline_marker_from_3(v0.clone(), v7);
                    let v11: bool = v1 == v10;
                    let v15: Rc<str> = if v11 {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v12.clone()
                    } else {
                        let v13: i32 = v10.wrapping_sub(1i32);
                        let v14: Rc<str> = string_slice(&v0.clone(), v1 as i64, v13 as i64);
                        v14.clone()
                    };
                    let v16: i32 = 0i32;
                    let v17: i32 = 0i32;
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = escape_from_0(v15.clone(), v16, v17, v18.clone());
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v19));
                    (v0, v1, v2) = (v0.clone(), v10, v20.clone());
                    continue;
                } else {
                    let v22: i32 = v8.wrapping_add(1i32);
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<code>"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v23));
                    let v25: i32 = v8.wrapping_sub(1i32);
                    let v26: Rc<str> = string_slice(&v0.clone(), v7 as i64, v25 as i64);
                    let v27: i32 = 0i32;
                    let v28: i32 = 0i32;
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = escape_from_0(v26.clone(), v27, v28, v29.clone());
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v30));
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code>"); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    (v0, v1, v2) = (v0.clone(), v22, v33.clone());
                    continue;
                }
            } else {
                let v36: bool = v5 == b'\\';
                if v36 {
                    let v37: i32 = v1.wrapping_add(1i32);
                    let v38: bool = v37 == v3;
                    if v38 {
                        let v39: i32 = next_inline_marker_from_3(v0.clone(), v37);
                        let v40: bool = v1 == v39;
                        let v44: Rc<str> = if v40 {
                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v41.clone()
                        } else {
                            let v42: i32 = v39.wrapping_sub(1i32);
                            let v43: Rc<str> = string_slice(&v0.clone(), v1 as i64, v42 as i64);
                            v43.clone()
                        };
                        let v45: i32 = 0i32;
                        let v46: i32 = 0i32;
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = escape_from_0(v44.clone(), v45, v46, v47.clone());
                        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v48));
                        (v0, v1, v2) = (v0.clone(), v39, v49.clone());
                        continue;
                    } else {
                        let v51: u8 = v0.clone().as_bytes()[v37 as usize];
                        let v52: bool = v51 == b'*';
                        let v70: bool = if v52 {
                            true
                        } else {
                            let v53: bool = v51 == b'_';
                            if v53 {
                                true
                            } else {
                                let v54: bool = v51 == b'\\';
                                if v54 {
                                    true
                                } else {
                                    let v55: bool = v51 == b'`';
                                    if v55 {
                                        true
                                    } else {
                                        let v56: bool = v51 == b'[';
                                        if v56 {
                                            true
                                        } else {
                                            let v57: bool = v51 == b']';
                                            if v57 {
                                                true
                                            } else {
                                                let v58: bool = v51 == b'#';
                                                if v58 {
                                                    true
                                                } else {
                                                    let v59: bool = v51 == b'-';
                                                    if v59 {
                                                        true
                                                    } else {
                                                        let v60: bool = v51 == b'!';
                                                        if v60 {
                                                            true
                                                        } else {
                                                            let v61: bool = v51 == b'<';
                                                            v61
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        if v70 {
                            let v71: i32 = v1.wrapping_add(2i32);
                            let v72: bool = v37 == v71;
                            let v76: Rc<str> = if v72 {
                                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v73.clone()
                            } else {
                                let v74: i32 = v71.wrapping_sub(1i32);
                                let v75: Rc<str> = string_slice(&v0.clone(), v37 as i64, v74 as i64);
                                v75.clone()
                            };
                            let v77: i32 = 0i32;
                            let v78: i32 = 0i32;
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v80: Rc<str> = escape_from_0(v76.clone(), v77, v78, v79.clone());
                            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v80));
                            (v0, v1, v2) = (v0.clone(), v71, v81.clone());
                            continue;
                        } else {
                            let v83: i32 = next_inline_marker_from_3(v0.clone(), v37);
                            let v84: bool = v1 == v83;
                            let v88: Rc<str> = if v84 {
                                let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v85.clone()
                            } else {
                                let v86: i32 = v83.wrapping_sub(1i32);
                                let v87: Rc<str> = string_slice(&v0.clone(), v1 as i64, v86 as i64);
                                v87.clone()
                            };
                            let v89: i32 = 0i32;
                            let v90: i32 = 0i32;
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v92: Rc<str> = escape_from_0(v88.clone(), v89, v90, v91.clone());
                            let v93: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v92));
                            (v0, v1, v2) = (v0.clone(), v83, v93.clone());
                            continue;
                        }
                    }
                } else {
                    let v97: bool = v5 == b'*';
                    if v97 {
                        let v98: i32 = v1.wrapping_add(1i32);
                        let v99: bool = v98 == v3;
                        if v99 {
                            let v100: i32 = next_inline_marker_from_3(v0.clone(), v98);
                            let v101: bool = v1 == v100;
                            let v105: Rc<str> = if v101 {
                                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v102.clone()
                            } else {
                                let v103: i32 = v100.wrapping_sub(1i32);
                                let v104: Rc<str> = string_slice(&v0.clone(), v1 as i64, v103 as i64);
                                v104.clone()
                            };
                            let v106: i32 = 0i32;
                            let v107: i32 = 0i32;
                            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v109: Rc<str> = escape_from_0(v105.clone(), v106, v107, v108.clone());
                            let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v109));
                            (v0, v1, v2) = (v0.clone(), v100, v110.clone());
                            continue;
                        } else {
                            let v112: u8 = v0.clone().as_bytes()[v98 as usize];
                            let v113: bool = v112 == b'*';
                            if v113 {
                                let v114: i32 = v1.wrapping_add(2i32);
                                let v115: i32 = double_close_4(v0.clone(), v114);
                                let v116: bool = v115 == -1i32;
                                if v116 {
                                    let v117: i32 = next_inline_marker_from_3(v0.clone(), v98);
                                    let v118: bool = v1 == v117;
                                    let v122: Rc<str> = if v118 {
                                        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v119.clone()
                                    } else {
                                        let v120: i32 = v117.wrapping_sub(1i32);
                                        let v121: Rc<str> = string_slice(&v0.clone(), v1 as i64, v120 as i64);
                                        v121.clone()
                                    };
                                    let v123: i32 = 0i32;
                                    let v124: i32 = 0i32;
                                    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v126: Rc<str> = escape_from_0(v122.clone(), v123, v124, v125.clone());
                                    let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v126));
                                    (v0, v1, v2) = (v0.clone(), v117, v127.clone());
                                    continue;
                                } else {
                                    let v129: i32 = v115.wrapping_add(2i32);
                                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<strong>"); } LIT.with(|lit| lit.clone()) };
                                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v130));
                                    let v132: bool = v114 == v115;
                                    let v136: Rc<str> = if v132 {
                                        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v133.clone()
                                    } else {
                                        let v134: i32 = v115.wrapping_sub(1i32);
                                        let v135: Rc<str> = string_slice(&v0.clone(), v114 as i64, v134 as i64);
                                        v135.clone()
                                    };
                                    let v137: i32 = 0i32;
                                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v139: Rc<str> = inline_from_1(v136.clone(), v137, v138.clone());
                                    let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v139));
                                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</strong>"); } LIT.with(|lit| lit.clone()) };
                                    let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
                                    (v0, v1, v2) = (v0.clone(), v129, v142.clone());
                                    continue;
                                }
                            } else {
                                let v145: i32 = next_inline_marker_from_3(v0.clone(), v98);
                                let v146: bool = v1 == v145;
                                let v150: Rc<str> = if v146 {
                                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v147.clone()
                                } else {
                                    let v148: i32 = v145.wrapping_sub(1i32);
                                    let v149: Rc<str> = string_slice(&v0.clone(), v1 as i64, v148 as i64);
                                    v149.clone()
                                };
                                let v151: i32 = 0i32;
                                let v152: i32 = 0i32;
                                let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v154: Rc<str> = escape_from_0(v150.clone(), v151, v152, v153.clone());
                                let v155: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v154));
                                (v0, v1, v2) = (v0.clone(), v145, v155.clone());
                                continue;
                            }
                        }
                    } else {
                        let v159: bool = v5 == b'<';
                        if v159 {
                            let v160: u8 = b'>';
                            let v161: i32 = v1.wrapping_add(1i32);
                            let v162: i32 = find_char_from_2(v0.clone(), v160, v161);
                            let v163: bool = v162 == -1i32;
                            if v163 {
                                let v164: i32 = next_inline_marker_from_3(v0.clone(), v161);
                                let v165: bool = v1 == v164;
                                let v169: Rc<str> = if v165 {
                                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v166.clone()
                                } else {
                                    let v167: i32 = v164.wrapping_sub(1i32);
                                    let v168: Rc<str> = string_slice(&v0.clone(), v1 as i64, v167 as i64);
                                    v168.clone()
                                };
                                let v170: i32 = 0i32;
                                let v171: i32 = 0i32;
                                let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v173: Rc<str> = escape_from_0(v169.clone(), v170, v171, v172.clone());
                                let v174: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v173));
                                (v0, v1, v2) = (v0.clone(), v164, v174.clone());
                                continue;
                            } else {
                                let v176: i32 = v1.wrapping_add(5i32);
                                let v177: bool = v162 > v176;
                                if v177 {
                                    let v178: bool = v161 == v162;
                                    let v182: Rc<str> = if v178 {
                                        let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v179.clone()
                                    } else {
                                        let v180: i32 = v162.wrapping_sub(1i32);
                                        let v181: Rc<str> = string_slice(&v0.clone(), v161 as i64, v180 as i64);
                                        v181.clone()
                                    };
                                    let v183: Rc<str> = string_slice(&v182.clone(), 0i32 as i64, 3i32 as i64);
                                    let v184: bool = &*v183 == "http";
                                    if v184 {
                                        let v185: i32 = v162.wrapping_add(1i32);
                                        let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
                                        let v187: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v186));
                                        let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                        let v189: Rc<str> = Rc::<str>::from(format!("{}{}", v187, v188));
                                        let v190: i32 = 0i32;
                                        let v191: i32 = 0i32;
                                        let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v193: Rc<str> = escape_from_0(v182.clone(), v190, v191, v192.clone());
                                        let v195: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v189, v193)), v188));
                                        let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                        let v197: Rc<str> = Rc::<str>::from(format!("{}{}", v195, v196));
                                        let v198: i32 = 0i32;
                                        let v199: i32 = 0i32;
                                        let v200: Rc<str> = escape_from_0(v182.clone(), v198, v199, v192.clone());
                                        let v201: Rc<str> = Rc::<str>::from(format!("{}{}", v197, v200));
                                        let v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
                                        let v203: Rc<str> = Rc::<str>::from(format!("{}{}", v201, v202));
                                        (v0, v1, v2) = (v0.clone(), v185, v203.clone());
                                        continue;
                                    } else {
                                        let v205: i32 = next_inline_marker_from_3(v0.clone(), v161);
                                        let v206: bool = v1 == v205;
                                        let v210: Rc<str> = if v206 {
                                            let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v207.clone()
                                        } else {
                                            let v208: i32 = v205.wrapping_sub(1i32);
                                            let v209: Rc<str> = string_slice(&v0.clone(), v1 as i64, v208 as i64);
                                            v209.clone()
                                        };
                                        let v211: i32 = 0i32;
                                        let v212: i32 = 0i32;
                                        let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v214: Rc<str> = escape_from_0(v210.clone(), v211, v212, v213.clone());
                                        let v215: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v214));
                                        (v0, v1, v2) = (v0.clone(), v205, v215.clone());
                                        continue;
                                    }
                                } else {
                                    let v218: i32 = next_inline_marker_from_3(v0.clone(), v161);
                                    let v219: bool = v1 == v218;
                                    let v223: Rc<str> = if v219 {
                                        let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v220.clone()
                                    } else {
                                        let v221: i32 = v218.wrapping_sub(1i32);
                                        let v222: Rc<str> = string_slice(&v0.clone(), v1 as i64, v221 as i64);
                                        v222.clone()
                                    };
                                    let v224: i32 = 0i32;
                                    let v225: i32 = 0i32;
                                    let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v227: Rc<str> = escape_from_0(v223.clone(), v224, v225, v226.clone());
                                    let v228: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v227));
                                    (v0, v1, v2) = (v0.clone(), v218, v228.clone());
                                    continue;
                                }
                            }
                        } else {
                            let v232: bool = v5 == b'!';
                            if v232 {
                                let v233: i32 = image_close_5(v0.clone(), v1);
                                let v234: bool = v233 == -1i32;
                                if v234 {
                                    let v235: i32 = v1.wrapping_add(1i32);
                                    let v236: i32 = next_inline_marker_from_3(v0.clone(), v235);
                                    let v237: bool = v1 == v236;
                                    let v241: Rc<str> = if v237 {
                                        let v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v238.clone()
                                    } else {
                                        let v239: i32 = v236.wrapping_sub(1i32);
                                        let v240: Rc<str> = string_slice(&v0.clone(), v1 as i64, v239 as i64);
                                        v240.clone()
                                    };
                                    let v242: i32 = 0i32;
                                    let v243: i32 = 0i32;
                                    let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v245: Rc<str> = escape_from_0(v241.clone(), v242, v243, v244.clone());
                                    let v246: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v245));
                                    (v0, v1, v2) = (v0.clone(), v236, v246.clone());
                                    continue;
                                } else {
                                    let v248: i32 = v233.wrapping_add(1i32);
                                    let v249: u8 = b']';
                                    let v250: i32 = v1.wrapping_add(2i32);
                                    let v251: i32 = find_char_from_2(v0.clone(), v249, v250);
                                    let v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src="); } LIT.with(|lit| lit.clone()) };
                                    let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                    let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src=\""); } LIT.with(|lit| lit.clone()) };
                                    let v255: i32 = v251.wrapping_add(2i32);
                                    let v256: bool = v255 == v233;
                                    let v260: Rc<str> = if v256 {
                                        let v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v257.clone()
                                    } else {
                                        let v258: i32 = v233.wrapping_sub(1i32);
                                        let v259: Rc<str> = string_slice(&v0.clone(), v255 as i64, v258 as i64);
                                        v259.clone()
                                    };
                                    let v261: i32 = 0i32;
                                    let v262: i32 = 0i32;
                                    let v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v264: Rc<str> = escape_from_0(v260.clone(), v261, v262, v263.clone());
                                    let v266: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v254, v264)), v253));
                                    let v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" alt="); } LIT.with(|lit| lit.clone()) };
                                    let v269: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v266, v267)), v253));
                                    let v270: bool = v250 == v251;
                                    let v273: Rc<str> = if v270 {
                                        v263.clone()
                                    } else {
                                        let v271: i32 = v251.wrapping_sub(1i32);
                                        let v272: Rc<str> = string_slice(&v0.clone(), v250 as i64, v271 as i64);
                                        v272.clone()
                                    };
                                    let v274: i32 = 0i32;
                                    let v275: i32 = 0i32;
                                    let v276: Rc<str> = escape_from_0(v273.clone(), v274, v275, v263.clone());
                                    let v278: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v269, v276)), v253));
                                    let v279: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                    let v281: Rc<str> = Rc::<str>::from(format!("{}{}", v2, Rc::<str>::from(format!("{}{}", v278, v279))));
                                    (v0, v1, v2) = (v0.clone(), v248, v281.clone());
                                    continue;
                                }
                            } else {
                                let v284: i32 = badge_close_6(v0.clone(), v1);
                                let v285: bool = v284 == -1i32;
                                if v285 {
                                    let v286: bool = v5 == b'[';
                                    if v286 {
                                        let v287: u8 = b']';
                                        let v288: i32 = v1.wrapping_add(1i32);
                                        let v289: i32 = find_char_from_2(v0.clone(), v287, v288);
                                        let v290: bool = v289 == -1i32;
                                        if v290 {
                                            let v291: i32 = next_inline_marker_from_3(v0.clone(), v288);
                                            let v292: bool = v1 == v291;
                                            let v296: Rc<str> = if v292 {
                                                let v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                v293.clone()
                                            } else {
                                                let v294: i32 = v291.wrapping_sub(1i32);
                                                let v295: Rc<str> = string_slice(&v0.clone(), v1 as i64, v294 as i64);
                                                v295.clone()
                                            };
                                            let v297: i32 = 0i32;
                                            let v298: i32 = 0i32;
                                            let v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v300: Rc<str> = escape_from_0(v296.clone(), v297, v298, v299.clone());
                                            let v301: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v300));
                                            (v0, v1, v2) = (v0.clone(), v291, v301.clone());
                                            continue;
                                        } else {
                                            let v303: i32 = v289.wrapping_add(1i32);
                                            let v304: bool = v303 == v3;
                                            if v304 {
                                                let v305: i32 = next_inline_marker_from_3(v0.clone(), v288);
                                                let v306: bool = v1 == v305;
                                                let v310: Rc<str> = if v306 {
                                                    let v307: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    v307.clone()
                                                } else {
                                                    let v308: i32 = v305.wrapping_sub(1i32);
                                                    let v309: Rc<str> = string_slice(&v0.clone(), v1 as i64, v308 as i64);
                                                    v309.clone()
                                                };
                                                let v311: i32 = 0i32;
                                                let v312: i32 = 0i32;
                                                let v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let v314: Rc<str> = escape_from_0(v310.clone(), v311, v312, v313.clone());
                                                let v315: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v314));
                                                (v0, v1, v2) = (v0.clone(), v305, v315.clone());
                                                continue;
                                            } else {
                                                let v317: u8 = v0.clone().as_bytes()[v303 as usize];
                                                let v318: bool = v317 == b'(';
                                                if v318 {
                                                    let v319: u8 = b')';
                                                    let v320: i32 = v289.wrapping_add(2i32);
                                                    let v321: i32 = find_char_from_2(v0.clone(), v319, v320);
                                                    let v322: bool = v321 == -1i32;
                                                    if v322 {
                                                        let v323: i32 = next_inline_marker_from_3(v0.clone(), v288);
                                                        let v324: bool = v1 == v323;
                                                        let v328: Rc<str> = if v324 {
                                                            let v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            v325.clone()
                                                        } else {
                                                            let v326: i32 = v323.wrapping_sub(1i32);
                                                            let v327: Rc<str> = string_slice(&v0.clone(), v1 as i64, v326 as i64);
                                                            v327.clone()
                                                        };
                                                        let v329: i32 = 0i32;
                                                        let v330: i32 = 0i32;
                                                        let v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        let v332: Rc<str> = escape_from_0(v328.clone(), v329, v330, v331.clone());
                                                        let v333: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v332));
                                                        (v0, v1, v2) = (v0.clone(), v323, v333.clone());
                                                        continue;
                                                    } else {
                                                        let v335: i32 = v289.wrapping_sub(1i32);
                                                        let v336: Rc<str> = string_slice(&v0.clone(), v288 as i64, v335 as i64);
                                                        let v337: i32 = v321.wrapping_sub(1i32);
                                                        let v338: Rc<str> = string_slice(&v0.clone(), v320 as i64, v337 as i64);
                                                        let v339: i32 = v321.wrapping_add(1i32);
                                                        let v340: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
                                                        let v341: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v340));
                                                        let v342: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                        let v343: Rc<str> = Rc::<str>::from(format!("{}{}", v341, v342));
                                                        let v344: i32 = 0i32;
                                                        let v345: i32 = 0i32;
                                                        let v346: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        let v347: Rc<str> = escape_from_0(v338.clone(), v344, v345, v346.clone());
                                                        let v349: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v343, v347)), v342));
                                                        let v350: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                                        let v351: Rc<str> = Rc::<str>::from(format!("{}{}", v349, v350));
                                                        let v352: i32 = 0i32;
                                                        let v353: i32 = 0i32;
                                                        let v354: Rc<str> = escape_from_0(v336.clone(), v352, v353, v346.clone());
                                                        let v355: Rc<str> = Rc::<str>::from(format!("{}{}", v351, v354));
                                                        let v356: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
                                                        let v357: Rc<str> = Rc::<str>::from(format!("{}{}", v355, v356));
                                                        (v0, v1, v2) = (v0.clone(), v339, v357.clone());
                                                        continue;
                                                    }
                                                } else {
                                                    let v360: i32 = next_inline_marker_from_3(v0.clone(), v288);
                                                    let v361: bool = v1 == v360;
                                                    let v365: Rc<str> = if v361 {
                                                        let v362: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v362.clone()
                                                    } else {
                                                        let v363: i32 = v360.wrapping_sub(1i32);
                                                        let v364: Rc<str> = string_slice(&v0.clone(), v1 as i64, v363 as i64);
                                                        v364.clone()
                                                    };
                                                    let v366: i32 = 0i32;
                                                    let v367: i32 = 0i32;
                                                    let v368: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    let v369: Rc<str> = escape_from_0(v365.clone(), v366, v367, v368.clone());
                                                    let v370: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v369));
                                                    (v0, v1, v2) = (v0.clone(), v360, v370.clone());
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        let v375: i32 = v1.wrapping_add(1i32);
                                        let v376: i32 = next_inline_marker_from_3(v0.clone(), v375);
                                        let v377: bool = v1 == v376;
                                        let v381: Rc<str> = if v377 {
                                            let v378: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v378.clone()
                                        } else {
                                            let v379: i32 = v376.wrapping_sub(1i32);
                                            let v380: Rc<str> = string_slice(&v0.clone(), v1 as i64, v379 as i64);
                                            v380.clone()
                                        };
                                        let v382: i32 = 0i32;
                                        let v383: i32 = 0i32;
                                        let v384: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v385: Rc<str> = escape_from_0(v381.clone(), v382, v383, v384.clone());
                                        let v386: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v385));
                                        (v0, v1, v2) = (v0.clone(), v376, v386.clone());
                                        continue;
                                    }
                                } else {
                                    let v389: i32 = badge_close_6(v0.clone(), v1);
                                    let v390: i32 = v1.wrapping_add(1i32);
                                    let v391: i32 = image_close_5(v0.clone(), v390);
                                    let v392: i32 = v389.wrapping_add(1i32);
                                    let v393: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
                                    let v394: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v393));
                                    let v395: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                    let v396: Rc<str> = Rc::<str>::from(format!("{}{}", v394, v395));
                                    let v397: i32 = v391.wrapping_add(3i32);
                                    let v398: bool = v397 == v389;
                                    let v402: Rc<str> = if v398 {
                                        let v399: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v399.clone()
                                    } else {
                                        let v400: i32 = v389.wrapping_sub(1i32);
                                        let v401: Rc<str> = string_slice(&v0.clone(), v397 as i64, v400 as i64);
                                        v401.clone()
                                    };
                                    let v403: i32 = 0i32;
                                    let v404: i32 = 0i32;
                                    let v405: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v406: Rc<str> = escape_from_0(v402.clone(), v403, v404, v405.clone());
                                    let v408: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v396, v406)), v395));
                                    let v409: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                    let v410: Rc<str> = Rc::<str>::from(format!("{}{}", v408, v409));
                                    let v411: u8 = b']';
                                    let v412: i32 = v390.wrapping_add(2i32);
                                    let v413: i32 = find_char_from_2(v0.clone(), v411, v412);
                                    let v414: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src="); } LIT.with(|lit| lit.clone()) };
                                    let v415: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src=\""); } LIT.with(|lit| lit.clone()) };
                                    let v416: i32 = v413.wrapping_add(2i32);
                                    let v417: bool = v416 == v391;
                                    let v420: Rc<str> = if v417 {
                                        v405.clone()
                                    } else {
                                        let v418: i32 = v391.wrapping_sub(1i32);
                                        let v419: Rc<str> = string_slice(&v0.clone(), v416 as i64, v418 as i64);
                                        v419.clone()
                                    };
                                    let v421: i32 = 0i32;
                                    let v422: i32 = 0i32;
                                    let v423: Rc<str> = escape_from_0(v420.clone(), v421, v422, v405.clone());
                                    let v425: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v415, v423)), v395));
                                    let v426: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" alt="); } LIT.with(|lit| lit.clone()) };
                                    let v428: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v425, v426)), v395));
                                    let v429: bool = v412 == v413;
                                    let v432: Rc<str> = if v429 {
                                        v405.clone()
                                    } else {
                                        let v430: i32 = v413.wrapping_sub(1i32);
                                        let v431: Rc<str> = string_slice(&v0.clone(), v412 as i64, v430 as i64);
                                        v431.clone()
                                    };
                                    let v433: i32 = 0i32;
                                    let v434: i32 = 0i32;
                                    let v435: Rc<str> = escape_from_0(v432.clone(), v433, v434, v405.clone());
                                    let v439: Rc<str> = Rc::<str>::from(format!("{}{}", v410, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v428, v435)), v395)), v409))));
                                    let v440: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
                                    let v441: Rc<str> = Rc::<str>::from(format!("{}{}", v439, v440));
                                    (v0, v1, v2) = (v0.clone(), v392, v441.clone());
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
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        inline_from_1(v0.clone(), v1, v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_escape(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn dtl_render_inline(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
