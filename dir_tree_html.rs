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
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_MissingFlag(Rc<str>),
    US2_BadNumber(Rc<str>, Rc<str>),
    US2_UnknownVerb(Rc<str>, Rc<UH0>),
    US2_NoVerb(Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US3 {
    US3_Ok(Rc<str>, Rc<str>),
    US3_Error(US2),
}
enum US4 {
    US4_Ok(Vec<u8>),
    US4_Error(std::io::Error),
}
#[derive(Clone)]
enum US5 {
    US5_Ok(std::string::String),
    US5_Error(std::string::FromUtf8Error),
}
fn command_line_argument_count_1() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn args_from_0(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_1();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_2(v0);
        US0::US0_Some(v5.clone())
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = args_from_0(v11);
            Rc::new(UH0::UH0_Cons(v10.clone(), v12.clone()))
        }
    }
}
fn walk_3(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v2 == &*v0;
                if v4 {
                    return true;
                } else {
                    (v0, v1) = (v0.clone(), v3.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn walk_4(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == &*v0;
                        if v6 {
                            return US0::US0_Some(v4.clone());
                        } else {
                            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                            (v0, v1) = (v0.clone(), v8.clone());
                            continue;
                        }
                    }
                    _ => {
                        return US0::US0_None;
                    }
                }
            }
            _ => {
                return US0::US0_None;
            }
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v1 == "";
                let v8: Rc<str> = if v4 {
                    v2.clone()
                } else {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | "); } LIT.with(|lit| lit.clone()) };
                    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v5)), v2));
                    v7.clone()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn ok_c0() -> Rc<dyn Fn(Vec<u8>) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<u8>) -> US4> = Rc::new(move |mut v0: Vec<u8>| -> US4 {
        US4::US4_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6() -> Rc<dyn Fn(Vec<u8>) -> US4> {
    ok_c0()
}
fn error_c1() -> Rc<dyn Fn(std::io::Error) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US4> = Rc::new(move |mut v0: std::io::Error| -> US4 {
        US4::US4_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7() -> Rc<dyn Fn(std::io::Error) -> US4> {
    error_c1()
}
fn method8(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn ok_c2() -> Rc<dyn Fn(std::string::String) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::String) -> US5> = Rc::new(move |mut v0: std::string::String| -> US5 {
        US5::US5_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(std::string::String) -> US5> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::FromUtf8Error) -> US5> = Rc::new(move |mut v0: std::string::FromUtf8Error| -> US5 {
        US5::US5_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US5> {
    error_c3()
}
fn last_sep_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
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
fn spiral_main() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = args_from_0(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("self-test"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--self-test"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = walk_3(v4.clone(), v1.clone());
    if v5 {
        let v6: i32 = dtl_self_test::dtl_self_test(0i32);
        v6
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
        let v9: US0 = walk_4(v8.clone(), v1.clone());
        let v15: US1 = match &v9 {
            US0::US0_None => {
                let v12: US2 = US2::US2_MissingFlag(v7.clone());
                US1::US1_Some(v12.clone())
            }
            US0::US0_Some(v10) => {
                let v10: Rc<str> = v10.clone();
                US1::US1_None
            }
        };
        let v27: US1 = match &v15 {
            US1::US1_None => {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                let v19: US0 = walk_4(v18.clone(), v1.clone());
                match &v19 {
                    US0::US0_None => {
                        let v22: US2 = US2::US2_MissingFlag(v17.clone());
                        US1::US1_Some(v22.clone())
                    }
                    US0::US0_Some(v20) => {
                        let v20: Rc<str> = v20.clone();
                        US1::US1_None
                    }
                }
            }
            US1::US1_Some(v16) => {
                let v16: US2 = v16.clone();
                v15.clone()
            }
        };
        let v49: US3 = match &v27 {
            US1::US1_None => {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
                let v31: US0 = walk_4(v30.clone(), v1.clone());
                let v37: Rc<str> = match &v31 {
                    US0::US0_None => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --dir"); } LIT.with(|lit| lit.clone()) };
                        std::panic::panic_any::<std::string::String>(format!("{}", v34.clone()))
                    }
                    US0::US0_Some(v32) => {
                        let v32: Rc<str> = v32.clone();
                        v32.clone()
                    }
                };
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                let v40: US0 = walk_4(v39.clone(), v1.clone());
                let v46: Rc<str> = match &v40 {
                    US0::US0_None => {
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --html"); } LIT.with(|lit| lit.clone()) };
                        std::panic::panic_any::<std::string::String>(format!("{}", v43.clone()))
                    }
                    US0::US0_Some(v41) => {
                        let v41: Rc<str> = v41.clone();
                        v41.clone()
                    }
                };
                US3::US3_Ok(v37.clone(), v46.clone())
            }
            US1::US1_Some(v28) => {
                let v28: US2 = v28.clone();
                US3::US3_Error(v28.clone())
            }
        };
        match &v49 {
            US3::US3_Error(v132) => {
                let v132: US2 = v132.clone();
                let v164: Rc<str> = match &v132 {
                    US2::US2_BadNumber(v138, v139) => {
                        let v138: Rc<str> = v138.clone();
                        let v139: Rc<str> = v139.clone();
                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: --"); } LIT.with(|lit| lit.clone()) };
                        let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v138));
                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects an integer, got '"); } LIT.with(|lit| lit.clone()) };
                        let v144: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v141, v142)), v139));
                        let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v146: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v145));
                        v146.clone()
                    }
                    US2::US2_MissingFlag(v133) => {
                        let v133: Rc<str> = v133.clone();
                        let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: missing --"); } LIT.with(|lit| lit.clone()) };
                        let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v133));
                        let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <value>"); } LIT.with(|lit| lit.clone()) };
                        let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
                        v137.clone()
                    }
                    US2::US2_NoVerb(v156) => {
                        let v156: Rc<UH0> = v156.clone();
                        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v158: Rc<str> = method5(v156.clone(), v157.clone());
                        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: no command given\nvalid: "); } LIT.with(|lit| lit.clone()) };
                        let v160: Rc<str> = Rc::<str>::from(format!("{}{}", v159, v158));
                        v160.clone()
                    }
                    US2::US2_UnknownVerb(v147, v148) => {
                        let v147: Rc<str> = v147.clone();
                        let v148: Rc<UH0> = v148.clone();
                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: unknown command '"); } LIT.with(|lit| lit.clone()) };
                        let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v147));
                        let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\nvalid: "); } LIT.with(|lit| lit.clone()) };
                        let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v151));
                        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v154: Rc<str> = method5(v148.clone(), v153.clone());
                        let v155: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v154));
                        v155.clone()
                    }
                };
                eprintln!("{}", v164);
                2i32
            }
            US3::US3_Ok(v50, v51) => {
                let v50: Rc<str> = v50.clone();
                let v51: Rc<str> = v51.clone();
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = Rc::<str>::from(std::path::Path::new(v50.as_ref()).join(v53.as_ref()).display().to_string());
                let v56: bool = std::path::Path::new(v54.as_ref()).is_file();
                let v99: Rc<str> = if v56 {
                    let v58: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v54);
                    let mut v59: Rc<dyn Fn(Vec<u8>) -> US4> = method6();
                    let mut v60: Rc<dyn Fn(std::io::Error) -> US4> = method7();
                    let v61: US4 = match v58 { Ok(x) => v59(x), Err(e) => v60(e) };
                    let v92: Rc<str> = match v61 {
                        US4::US4_Error(v62) => {
                            let v62: std::io::Error = v62;
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v63.clone()
                        }
                        US4::US4_Ok(v64) => {
                            let v64: Vec<u8> = v64.clone();
                            let v65: Vec<u8> = method8(v64.clone());
                            let v67: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v65);
                            let mut v68: Rc<dyn Fn(std::string::String) -> US5> = method9();
                            let mut v69: Rc<dyn Fn(std::string::FromUtf8Error) -> US5> = method10();
                            let v70: US5 = match v67 { Ok(x) => v68(x), Err(e) => v69(e) };
                            match &v70 {
                                US5::US5_Error(v71) => {
                                    let v71: std::string::FromUtf8Error = v71.clone();
                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v72.clone()
                                }
                                US5::US5_Ok(v73) => {
                                    let v73: std::string::String = v73.clone();
                                    let v75: Rc<str> = Rc::<str>::from(String::as_str(&v73));
                                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
                                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v78: Rc<str> = Rc::<str>::from(v75.replace(&*v76, &*v77));
                                    let v79: bool = v78.ends_with(&*v77);
                                    let v82: bool = if v79 {
                                        let v80: i32 = (v78.clone().len() as i32);
                                        let v81: bool = v80 == 1i32;
                                        v81
                                    } else {
                                        false
                                    };
                                    if v82 {
                                        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v83.clone()
                                    } else {
                                        if v79 {
                                            let v84: i32 = (v78.clone().len() as i32);
                                            let v85: i32 = v84.wrapping_sub(2i32);
                                            let v86: Rc<str> = string_slice(&v78.clone(), 0i32 as i64, v85 as i64);
                                            v86.clone()
                                        } else {
                                            v78.clone()
                                        }
                                    }
                                }
                            }
                        }
                    };
                    let v93: Rc<str> = dtl_markdown::dtl_markdown(&*v92);
                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v93));
                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
                    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
                    v97.clone()
                } else {
                    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v98.clone()
                };
                let v100: Rc<str> = dtl_tree::dtl_tree_html(&*v50);
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v100));
                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v99));
                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
                let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v102, Rc::<str>::from(format!("{}{}", v104, v105))));
                let v108: i32 = 0i32;
                let v109: i32 = -1i32;
                let v110: i32 = last_sep_11(v51.clone(), v108, v109);
                let v111: bool = v110 == -1i32;
                let v122: Rc<str> = if v111 {
                    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v112.clone()
                } else {
                    let v113: i32 = v110.wrapping_sub(1i32);
                    let v114: Rc<str> = string_slice(&v51.clone(), 0i32 as i64, v113 as i64);
                    let v115: i32 = (v114.clone().len() as i32);
                    let v116: bool = v115 == 2i32;
                    let v119: bool = if v116 {
                        let v117: u8 = v114.clone().as_bytes()[1i32 as usize];
                        let v118: bool = v117 == b':';
                        v118
                    } else {
                        false
                    };
                    if v119 {
                        let v120: Rc<str> = string_slice(&v51.clone(), 0i32 as i64, v110 as i64);
                        v120.clone()
                    } else {
                        v114.clone()
                    }
                };
                let v123: i32 = (v122.clone().len() as i32);
                let v124: bool = v123 == 0i32;
                let v127: i32 = if v124 {
                    0i32
                } else {
                    let v126: i32 = i32::from(std::fs::create_dir_all(v122.as_ref()).is_err());
                    v126
                };
                let v128: bool = v127 == 0i32;
                if v128 {
                    let v130: i32 = i32::from(std::fs::write(v51.as_ref(), v107.as_ref().as_bytes()).is_err());
                    v130
                } else {
                    v127
                }
            }
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
