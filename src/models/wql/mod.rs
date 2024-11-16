use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Types3 {
    hits: Hits,
    #[serde(rename = "_shards")]
    shards: Shards,
    timed_out: bool,
    took: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Hits {
    hits: Vec<Hit>,
    max_score: Option<serde_json::Value>,
    total: Total,
}

#[derive(Serialize, Deserialize)]
pub struct Hit {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_score")]
    score: Option<serde_json::Value>,
    sort: Vec<i64>,
    #[serde(rename = "_source")]
    source: Source,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    agent: Agent,
    data: Option<Data>,
    decoder: Decoder,
    full_log: Option<String>,
    id: String,
    input: Input,
    location: String,
    manager: Manager,
    previous_output: Option<String>,
    rule: Rule,
    #[serde(rename = "timestamp")]
    source_timestamp: String,
    syscheck: Option<Syscheck>,
    #[serde(rename = "@timestamp")]
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct Agent {
    id: String,
    ip: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    integration: Option<String>,
    level: Option<String>,
    virustotal: Option<Virustotal>,
    vulnerability: Option<Vulnerability>,
    win: Option<Win>,
}

#[derive(Serialize, Deserialize)]
pub struct Virustotal {
    found: String,
    malicious: String,
    permalink: String,
    positives: String,
    scan_date: String,
    sha1: String,
    source: SourceClass,
    total: String,
}

#[derive(Serialize, Deserialize)]
pub struct SourceClass {
    alert_id: String,
    file: String,
    md5: String,
    sha1: String,
}

#[derive(Serialize, Deserialize)]
pub struct Vulnerability {
    assigner: String,
    cve: String,
    cvss: Cvss,
    cwe_reference: Option<String>,
    enumeration: String,
    package: Package,
    published: String,
    rationale: String,
    reference: String,
    severity: String,
    status: String,
    title: String,
    updated: String,
    #[serde(rename = "type")]
    vulnerability_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Cvss {
    cvss2: Option<Cvss2>,
    cvss3: Option<Cvss3>,
}

#[derive(Serialize, Deserialize)]
pub struct Cvss2 {
    base_score: String,
    vector: Cvss2Vector,
}

#[derive(Serialize, Deserialize)]
pub struct Cvss2Vector {
    access_complexity: String,
    authentication: String,
    availability: String,
    confidentiality_impact: String,
    integrity_impact: String,
}

#[derive(Serialize, Deserialize)]
pub struct Cvss3 {
    base_score: String,
    vector: Cvss3Vector,
}

#[derive(Serialize, Deserialize)]
pub struct Cvss3Vector {
    attack_vector: Option<String>,
    availability: String,
    confidentiality_impact: String,
    integrity_impact: String,
    privileges_required: String,
    scope: String,
    user_interaction: String,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    architecture: String,
    condition: String,
    name: String,
    source: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Win {
    eventdata: Eventdata,
    system: System,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eventdata {
    account_name: Option<String>,
    authentication_package_name: Option<String>,
    binary: Option<String>,
    data: Option<String>,
    error_state: Option<String>,
    #[serde(rename = "type")]
    eventdata_type: Option<String>,
    failure_reason: Option<String>,
    image_path: Option<String>,
    ip_address: Option<String>,
    ip_port: Option<String>,
    key_length: Option<String>,
    library: Option<String>,
    logon_process_name: Option<String>,
    logon_type: Option<String>,
    name: Option<String>,
    param1: Option<String>,
    param2: Option<String>,
    param3: Option<String>,
    param4: Option<String>,
    process_id: Option<String>,
    service_name: Option<String>,
    service_type: Option<String>,
    start_type: Option<String>,
    status: Option<String>,
    sub_status: Option<String>,
    subject_logon_id: Option<String>,
    subject_user_sid: Option<String>,
    target_domain_name: Option<String>,
    target_user_name: Option<String>,
    target_user_sid: Option<String>,
    value: Option<String>,
    win32_error: Option<String>,
    workstation_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    channel: String,
    computer: String,
    #[serde(rename = "eventID")]
    event_id: String,
    #[serde(rename = "eventRecordID")]
    event_record_id: String,
    event_source_name: Option<String>,
    keywords: String,
    level: String,
    message: String,
    opcode: Option<String>,
    #[serde(rename = "processID")]
    process_id: Option<String>,
    provider_guid: Option<String>,
    provider_name: String,
    severity_value: String,
    system_time: String,
    task: String,
    #[serde(rename = "threadID")]
    thread_id: Option<String>,
    version: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Decoder {
    name: String,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "type")]
    input_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Manager {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
    description: String,
    firedtimes: i64,
    frequency: Option<i64>,
    gdpr: Option<Vec<String>>,
    groups: Vec<String>,
    hipaa: Option<Vec<String>>,
    id: String,
    level: i64,
    mail: bool,
    mitre: Option<Mitre>,
    nist_800_53: Option<Vec<String>>,
    pci_dss: Option<Vec<String>>,
    tsc: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Mitre {
    id: Vec<String>,
    tactic: Vec<String>,
    technique: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Syscheck {
    changed_attributes: Vec<String>,
    event: String,
    gid_after: String,
    gname_after: String,
    inode_after: i64,
    inode_before: i64,
    md5_after: String,
    mode: String,
    mtime_after: String,
    path: String,
    perm_after: String,
    sha1_after: String,
    sha256_after: String,
    size_after: String,
    uid_after: String,
    uname_after: String,
}

#[derive(Serialize, Deserialize)]
pub struct Total {
    relation: String,
    value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Shards {
    failed: i64,
    skipped: i64,
    successful: i64,
    total: i64,
}
