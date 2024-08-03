use std::{borrow::Cow, collections::HashMap};

/** Data Transfer Objects (DTO)
*  Example data:
* ```yaml
* Scoresheet:
   Scorer: 小四
   Archer: 橙月
   Equipments:
   - Riser: "WIAWIS ATF-DX SUN ORANGE"
   - Limbs: "WNS Delta C2"
   - Sight: "Shibuya ULTIMA RCIII 520"
   - Other stuff
   Metadata:
       Competition: "BC Provincial 2024 - Maple Ridge"
       Category: "Recurve 21+ Men"
       Rule: 脑残规则
   Sections:
   - Section:
       Metadata:
           Distance: "30m"
           Elevation: "5m"
           Target: "FITA 40cm"
       Rounds:
       - Round: ["X", 10, 9]
       - Round: [11, 10, 9]
   - Section:
       Metadata:
           Distance: "50m"
           Elevation: "10m"
           Target: "FITA 80cm"
       Rounds:
       - Round: ["X", 9, 8]
       - Round: [9, 8, 8]
   - Section:
       Metadata:
           Distance: "70m"
           Elevation: "0m"
           Target: "FITA 122cm"
       Rounds:
       - Round: [10, 8, 7]
       - Round: [9, 9, 9]
* ```
*/

pub struct Scoresheet<'a> {
    metadata: Cow<'a, Metadata>,
    scorer: String,
    archer: String,
    equipments: Cow<'a, Equipments>,
    sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    competition: String,
    category: String,
    rule: String,
}

#[derive(Debug, Clone)]
pub struct Equipments {
    riser: String,
    limbs: String,
    sight: String,
    others: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Section {
    metadata: SectionMetadata,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone)]
pub struct SectionMetadata {
    distance: String,
    elevation: String,
    target: String,
}

#[derive(Debug, Clone)]
pub struct Round {
    scores: Vec<u32>,
}
