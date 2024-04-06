#[test]
fn test() {

    use tcgatools::xml_clinic_merge;

    let file = "F:\\shenyi\\database\\rawdata\\TCGA\\clinic\\xml\\00a5e81c-cd67-483f-9d99-3c733b2ead38\\nationwidechildrens.org_clinical.TCGA-D8-A1JM.xml";
    let fil2 = "F:\\shenyi\\database\\rawdata\\TCGA\\clinic\\xml\\00e6f83a-36a8-4019-9fd9-e7e3e99c455b\\nationwidechildrens.org_clinical.TCGA-BB-4217.xml";
    let files = vec![file.to_string(), fil2.to_string()];
    let cols = vec!["hpv_status_by_ish_testing".to_string(), "lymphnode_neck_dissection".to_string()];
    xml_clinic_merge::xml_clinic_merge(files, Some(cols), ".\\123".to_string());
}
