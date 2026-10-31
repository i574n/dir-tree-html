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
enum US0 {
    US0_MdNone,
    US0_MdParagraph,
    US0_MdList,
    US0_MdCode,
}
fn find_nl_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'\n';
            if v5 {
                return v1;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            }
        }
    }
}
fn starts_at_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v2 == v3;
        if v4 {
            return true;
        } else {
            let v5: i32 = (v0.clone().len() as i32);
            let v6: bool = v2 == v5;
            if v6 {
                return false;
            } else {
                let v7: u8 = v0.clone().as_bytes()[v2 as usize];
                let v8: u8 = v1.clone().as_bytes()[v2 as usize];
                let v9: bool = v7 == v8;
                if v9 {
                    let v10: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v1.clone(), v10);
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn count_hashes_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'#';
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn blank_from_4(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return true;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b' ';
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn lead_spaces_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b' ';
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn markdown_lines_0(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 > v4;
        if v5 {
            let v13: Rc<str> = match &v2 {
                US0::US0_MdCode => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
                US0::US0_MdList => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US0::US0_MdNone => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US0::US0_MdParagraph => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
            };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v13));
            return v14.clone();
        } else {
            let v15: i32 = find_nl_1(v0.clone(), v1);
            let v16: i32 = v15.wrapping_sub(1i32);
            let v17: Rc<str> = string_slice(&v0.clone(), v1 as i64, v16 as i64);
            let v18: i32 = v15.wrapping_add(1i32);
            match &v2 {
                US0::US0_MdCode => {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                    let v20: i32 = 0i32;
                    let v21: bool = starts_at_2(v17.clone(), v19.clone(), v20);
                    if v21 {
                        let v22: US0 = US0::US0_MdNone;
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v23));
                        (v0, v1, v2, v3) = (v0.clone(), v18, v22.clone(), v24.clone());
                        continue;
                    } else {
                        let v26: US0 = US0::US0_MdCode;
                        let v27: Rc<str> = dtl_inline::dtl_escape(&*v17);
                        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v27));
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                        (v0, v1, v2, v3) = (v0.clone(), v18, v26.clone(), v30.clone());
                        continue;
                    }
                }
                _ => {
                    let v33: i32 = 0i32;
                    let v34: i32 = count_hashes_3(v17.clone(), v33);
                    let v35: bool = v34 == 0i32;
                    let v44: i32 = if v35 {
                        0i32
                    } else {
                        let v36: bool = v34 > 6i32;
                        if v36 {
                            0i32
                        } else {
                            let v37: i32 = (v17.clone().len() as i32);
                            let v38: bool = v34 == v37;
                            if v38 {
                                0i32
                            } else {
                                let v39: u8 = v17.clone().as_bytes()[v34 as usize];
                                let v40: bool = v39 == b' ';
                                if v40 {
                                    v34
                                } else {
                                    0i32
                                }
                            }
                        }
                    };
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                    let v46: i32 = 0i32;
                    let v47: bool = starts_at_2(v17.clone(), v45.clone(), v46);
                    if v47 {
                        let v48: US0 = US0::US0_MdCode;
                        let v54: Rc<str> = match &v2 {
                            US0::US0_MdList => {
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US0::US0_MdNone => {
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                            US0::US0_MdParagraph => {
                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            _ => unreachable!(),
                        };
                        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v54));
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
                        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                        (v0, v1, v2, v3) = (v0.clone(), v18, v48.clone(), v57.clone());
                        continue;
                    } else {
                        let v59: i32 = 0i32;
                        let v60: bool = blank_from_4(v17.clone(), v59);
                        if v60 {
                            let v61: US0 = US0::US0_MdNone;
                            let v67: Rc<str> = match &v2 {
                                US0::US0_MdList => {
                                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                    v64.clone()
                                }
                                US0::US0_MdNone => {
                                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v62.clone()
                                }
                                US0::US0_MdParagraph => {
                                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                    v63.clone()
                                }
                                _ => unreachable!(),
                            };
                            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v67));
                            (v0, v1, v2, v3) = (v0.clone(), v18, v61.clone(), v68.clone());
                            continue;
                        } else {
                            let v70: bool = v44 > 0i32;
                            if v70 {
                                let v71: bool = v44 == 1i32;
                                let v86: Rc<str> = if v71 {
                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1"); } LIT.with(|lit| lit.clone()) };
                                    v72.clone()
                                } else {
                                    let v73: bool = v44 == 2i32;
                                    if v73 {
                                        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h2"); } LIT.with(|lit| lit.clone()) };
                                        v74.clone()
                                    } else {
                                        let v75: bool = v44 == 3i32;
                                        if v75 {
                                            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h3"); } LIT.with(|lit| lit.clone()) };
                                            v76.clone()
                                        } else {
                                            let v77: bool = v44 == 4i32;
                                            if v77 {
                                                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h4"); } LIT.with(|lit| lit.clone()) };
                                                v78.clone()
                                            } else {
                                                let v79: bool = v44 == 5i32;
                                                if v79 {
                                                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h5"); } LIT.with(|lit| lit.clone()) };
                                                    v80.clone()
                                                } else {
                                                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h6"); } LIT.with(|lit| lit.clone()) };
                                                    v81.clone()
                                                }
                                            }
                                        }
                                    }
                                };
                                let v87: i32 = v44.wrapping_add(1i32);
                                let v88: i32 = (v17.clone().len() as i32);
                                let v89: i32 = v88.wrapping_sub(1i32);
                                let v90: Rc<str> = string_slice(&v17.clone(), v87 as i64, v89 as i64);
                                let v91: US0 = US0::US0_MdNone;
                                let v97: Rc<str> = match &v2 {
                                    US0::US0_MdList => {
                                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                        v94.clone()
                                    }
                                    US0::US0_MdNone => {
                                        let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v92.clone()
                                    }
                                    US0::US0_MdParagraph => {
                                        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                        v93.clone()
                                    }
                                    _ => unreachable!(),
                                };
                                let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v97));
                                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                                let v101: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v98, v99)), v86));
                                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                                let v104: Rc<str> = dtl_inline::dtl_render_inline(&*v90);
                                let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v104));
                                let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
                                let v108: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v105, v106)), v86));
                                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">\n"); } LIT.with(|lit| lit.clone()) };
                                let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v109));
                                (v0, v1, v2, v3) = (v0.clone(), v18, v91.clone(), v110.clone());
                                continue;
                            } else {
                                let v112: bool = &*v17 == "---";
                                if v112 {
                                    let v113: US0 = US0::US0_MdNone;
                                    let v119: Rc<str> = match &v2 {
                                        US0::US0_MdList => {
                                            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                            v116.clone()
                                        }
                                        US0::US0_MdNone => {
                                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v114.clone()
                                        }
                                        US0::US0_MdParagraph => {
                                            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                            v115.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v119));
                                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<hr>\n"); } LIT.with(|lit| lit.clone()) };
                                    let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
                                    (v0, v1, v2, v3) = (v0.clone(), v18, v113.clone(), v122.clone());
                                    continue;
                                } else {
                                    let v124: i32 = 0i32;
                                    let v125: i32 = lead_spaces_5(v17.clone(), v124);
                                    let v126: i32 = v125.wrapping_add(2i32);
                                    let v127: i32 = (v17.clone().len() as i32);
                                    let v128: bool = v126 > v127;
                                    let v138: i32 = if v128 {
                                        -1i32
                                    } else {
                                        let v129: i32 = v125.wrapping_add(1i32);
                                        let v130: u8 = v17.clone().as_bytes()[v129 as usize];
                                        let v131: bool = v130 == b' ';
                                        if v131 {
                                            let v132: u8 = v17.clone().as_bytes()[v125 as usize];
                                            let v133: bool = v132 == b'-';
                                            if v133 {
                                                v126
                                            } else {
                                                let v134: bool = v132 == b'*';
                                                if v134 {
                                                    v126
                                                } else {
                                                    -1i32
                                                }
                                            }
                                        } else {
                                            -1i32
                                        }
                                    };
                                    let v139: bool = v138 > 0i32;
                                    if v139 {
                                        let v140: i32 = 0i32;
                                        let v141: i32 = lead_spaces_5(v17.clone(), v140);
                                        let v142: i32 = v141.wrapping_add(2i32);
                                        let v143: bool = v142 > v127;
                                        let v153: i32 = if v143 {
                                            -1i32
                                        } else {
                                            let v144: i32 = v141.wrapping_add(1i32);
                                            let v145: u8 = v17.clone().as_bytes()[v144 as usize];
                                            let v146: bool = v145 == b' ';
                                            if v146 {
                                                let v147: u8 = v17.clone().as_bytes()[v141 as usize];
                                                let v148: bool = v147 == b'-';
                                                if v148 {
                                                    v142
                                                } else {
                                                    let v149: bool = v147 == b'*';
                                                    if v149 {
                                                        v142
                                                    } else {
                                                        -1i32
                                                    }
                                                }
                                            } else {
                                                -1i32
                                            }
                                        };
                                        let v154: bool = v153 == v127;
                                        let v158: Rc<str> = if v154 {
                                            let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v155.clone()
                                        } else {
                                            let v156: i32 = v127.wrapping_sub(1i32);
                                            let v157: Rc<str> = string_slice(&v17.clone(), v153 as i64, v156 as i64);
                                            v157.clone()
                                        };
                                        let v159: Rc<str> = dtl_inline::dtl_render_inline(&*v158);
                                        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) };
                                        let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v159));
                                        let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) };
                                        let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v162));
                                        match &v2 {
                                            US0::US0_MdList => {
                                                let v164: US0 = US0::US0_MdList;
                                                let v165: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v163));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v164.clone(), v165.clone());
                                                continue;
                                            }
                                            _ => {
                                                let v167: US0 = US0::US0_MdList;
                                                let v171: Rc<str> = match &v2 {
                                                    US0::US0_MdNone => {
                                                        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v168.clone()
                                                    }
                                                    US0::US0_MdParagraph => {
                                                        let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
                                                        v169.clone()
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v171));
                                                let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ul>\n"); } LIT.with(|lit| lit.clone()) };
                                                let v175: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v172, v173)), v163));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v167.clone(), v175.clone());
                                                continue;
                                            }
                                        }
                                    } else {
                                        match &v2 {
                                            US0::US0_MdParagraph => {
                                                let v178: US0 = US0::US0_MdParagraph;
                                                let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v179));
                                                let v181: Rc<str> = dtl_inline::dtl_render_inline(&*v17);
                                                let v182: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v181));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v178.clone(), v182.clone());
                                                continue;
                                            }
                                            _ => {
                                                let v184: US0 = US0::US0_MdParagraph;
                                                let v188: Rc<str> = match &v2 {
                                                    US0::US0_MdList => {
                                                        let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
                                                        v186.clone()
                                                    }
                                                    US0::US0_MdNone => {
                                                        let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v185.clone()
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                let v189: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v188));
                                                let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) };
                                                let v191: Rc<str> = Rc::<str>::from(format!("{}{}", v189, v190));
                                                let v192: Rc<str> = dtl_inline::dtl_render_inline(&*v17);
                                                let v193: Rc<str> = Rc::<str>::from(format!("{}{}", v191, v192));
                                                (v0, v1, v2, v3) = (v0.clone(), v18, v184.clone(), v193.clone());
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
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: US0 = US0::US0_MdNone;
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        markdown_lines_0(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_markdown(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
