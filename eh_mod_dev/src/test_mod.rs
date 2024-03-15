use crate::database::{Database};
use eh_schema::schema::{
    Ammunition, BulletBody, BulletControllerParametric, BulletPrefab, BulletPrefabId,
    ComponentStats, DamageType, ImpactEffect, ImpactEffectType,
};



pub fn build_mod() {
    let mut db = Database::new("./eh_mod_dev/database");

    db.add_id_range(9870000..9999999);
    db.set_id::<BulletPrefab>("eh:mine", 9);
    db.set_id::<ComponentStats>("eh:weapon", 1);

    parametric_ammo(&mut db);

    db.save();
}

fn parametric_ammo(db: &mut Database) {
    let id = db.add_item(Ammunition::new(db.id("juh9870:parametric")));
    let mut ammo = db.get_item_mut(id).unwrap();

    ammo.body = simple_body(db.id("eh:mine"), 5.0);

    ammo.controller = BulletControllerParametric::new()
        .with_x("t * 10")
        .with_y("SIN(t)")
        .into();

    ammo.effects.push(damage(DamageType::Energy, 10.0));

    // let component = db
    //     .add_item(Component::new(
    //         db.id("juh9870:parametric"),
    //         db.id("eh:weapon"),
    //     ))
    //     .item_mut(db);
}

fn damage(ty: DamageType, damage: f32) -> ImpactEffect {
    ImpactEffect {
        r#type: ImpactEffectType::Damage,
        damage_type: ty,
        power: damage,
        factor: 0.0,
    }
}

fn simple_body(prefab: BulletPrefabId, lifetime: f32) -> BulletBody {
    BulletBody::new()
        .with_lifetime(lifetime)
        .with_bullet_prefab(prefab)
}
