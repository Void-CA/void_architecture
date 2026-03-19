use super::FileType;

pub fn execute(feature_name: &str) {
    let feature_path = format!("src/features/{}", feature_name);

    let dirs = vec![
        FileType::Dir(format!("{}/components", feature_path)),
        FileType::Dir(format!("{}/hooks", feature_path)),
        FileType::Dir(format!("{}/data", feature_path)),
    ];

    let files = vec![
        // root
        FileType::File(format!("{}/index.ts", feature_path)),
        FileType::File(format!("{}/service.ts", feature_path)),
        FileType::File(format!("{}/queryKeys.ts", feature_path)),

        // hooks
        FileType::File(format!("{}/hooks/index.ts", feature_path)),
        FileType::File(format!("{}/hooks/queries.ts", feature_path)),

        // data
        FileType::File(format!("{}/data/dto.ts", feature_path)),
        FileType::File(format!("{}/data/model.ts", feature_path)),
        FileType::File(format!("{}/data/mapper.ts", feature_path)),
    ];

    // 1. Crear directorios
    for dir in dirs {
        dir.create();
    }

    // 2. Crear archivos
    for file in files {
        file.create();
    }
}