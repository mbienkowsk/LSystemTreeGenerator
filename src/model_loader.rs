use tobj::Model;

fn load_obj_file(path: &str) -> Model {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(models.len(), 1, "Expected exactly one model in {}", path);
    models.first().expect("Expected model").clone()
}

pub fn load_monkey() -> Model {
    load_obj_file("assets/models/monkey.obj")
}

pub fn load_cone() -> Model {
    load_obj_file("assets/models/cone.obj")
}

pub fn load_floor() -> Model {
    load_obj_file("assets/models/floor.obj")
}

#[cfg(test)]
mod tests {
    use tobj::Model;
    use crate::model_loader::load_monkey;

    fn display_model_info(model: &Model) {
        println!("Model name: {}", model.name);
        println!(
            "Loaded model with {} positions",
            model.mesh.positions.len()
        );
        println!(
            "Loaded model with {} indices",
            model.mesh.indices.len()
        );
        println!(
            "Loaded model with {} normals",
            model.mesh.normals.len()
        );
        println!(
            "Loaded model with {} texcoords",
            model.mesh.texcoords.len()
        );
    }

    fn check_if_model_loaded_correctly(model: &Model) {
        assert!(!model.mesh.positions.is_empty());
        assert!(!model.mesh.normals.is_empty());
        assert_eq!(
            model.mesh.positions.len(),
            model.mesh.normals.len()
        );
    }

    #[test]
    fn monkey_loads_correctly() {
        let monkey_model = load_monkey();

        display_model_info(&monkey_model);
        check_if_model_loaded_correctly(&monkey_model);
    }

    #[test]
    fn cone_loads_correctly() {
        let cone_model = crate::model_loader::load_cone();
        display_model_info(&cone_model);
        check_if_model_loaded_correctly(&cone_model);
    }

    #[test]
    fn floor_loads_correctly() {
        let floor_model = crate::model_loader::load_floor();
        display_model_info(&floor_model);
        check_if_model_loaded_correctly(&floor_model);
    }
}
