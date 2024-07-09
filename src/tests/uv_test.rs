use crate::to;

to!(
    catch_test_1,
    // Input codes
    r#"
    import {initMonitor} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
    }catch(err){
    }
    "#,
    // Output codes after transformed with plugin
    r#"
    import {initMonitor, weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
    }catch(err){
        weirwoodErrorReport(err)
    }
    "#
);
to!(
    catch_test_2,
    // Input codes
    r#"try{
        let a = 1;
    }catch(error){
        
    }"#,
    // Output codes after transformed with plugin
    r#"
    import {weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
    }catch(error){
        weirwoodErrorReport(error)
    }"#
);

to!(
    catch_test_3,
    // Input codes
    r#"try{
        let a = 1;
        let c = 2;
    }catch(error){
        let b = 2;
    }"#,
    // Output codes after transformed with plugin
    r#"
    import {weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
        let c = 2;
    }catch(error){
        weirwoodErrorReport(error)
        let b = 2;
    }"#
);

to!(
    catch_test_4,
    // Input codes
    r#"
    import {weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
        let c = 2;
    }catch(error){
        let b = 2;
        weirwoodErrorReport(error);
    }"#,
    // Output codes after transformed with plugin
    r#"
    import {weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
        let c = 2;
    }catch(error){
        let b = 2;
        weirwoodErrorReport(error);
    }"#
);

to!(
    catch_test_5,
    // Input codes
    r#"
    try{
        let a = 1;
        let c = 2;
        try {
            let d = 3;
        }catch(err){
            console.log(err);
        }
    }catch(error){
        let b = 2;
    }"#,
    // Output codes after transformed with plugin
    r#"
    import {weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
        let c = 2;
        try {
            let d = 3;
        }catch(err){
            weirwoodErrorReport(err);
            console.log(err);
        }
    }catch(error){
        weirwoodErrorReport(error);
        let b = 2;
    }"#
);

to!(
    catch_test_6,
    // Input codes
    r#"
    let a = 1;
    "#,
    // Output codes after transformed with plugin
    r#"
    let a = 1;
    "#
);
