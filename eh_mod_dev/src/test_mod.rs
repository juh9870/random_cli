use crate::database::{database, Database};
use eh_schema::schema::{
    BulletBody, BulletControllerParametric, BulletPrefab, BulletPrefabId, ComponentStats,
    DamageType, ImpactEffect, ImpactEffectType, Item,
};

pub fn build_mod() {
    let db = database("./eh_mod_dev/database");

    db.add_id_range(9870000..9999999);
    db.set_id::<BulletPrefab>("eh:mine", 9);
    db.set_id::<ComponentStats>("eh:weapon", 1);

    parametric_ammo(db.clone());

    db.save();
}

fn parametric_ammo(db: Database) {
    let mut ammo = db.add_item(Item::ammunition(db.id("juh9870:parametric")));

    ammo.body = simple_body(db.id("eh:mine"), 5.0);

    ammo.controller = BulletControllerParametric::new()
        .with_x("t * 10")
        .with_y("SIN(t)")
        .into();

    ammo.effects.push(damage(DamageType::Energy, 10.0));

    let mut component = db.add_item(Item::component(
        db.id("juh9870:parametric"),
        db.id("eh:weapon"),
    ));

    component.ammunition_id = Some(ammo.id);
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
