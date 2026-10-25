use std::io::Write;

fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let header = "Commissioner,Ministry,Zone\n";

    let mut file = std::fs::File::create("efcc_merged.csv").expect("create failed");
    file.write_all(header.as_bytes()).expect("write failed");

    for i in 0..commissioners.len() {
        file.write_all(commissioners[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(ministries[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(zones[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("Merged into efcc_merged.csv");
}