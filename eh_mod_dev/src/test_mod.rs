use crate::database::{database, Database, DatabaseIdLike, DbItem};
use eh_schema::schema::{
    ActivationType, BulletBody, BulletController, BulletPrefab, BulletPrefabId, ComponentStats,
    DamageType, ImpactEffect, ImpactEffectType, Weapon, WeaponClass,
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
    let ammo = db.ammunition("juh9870:parametric").edit(|ammo| {
        ammo.body = simple_body(db.id("eh:mine"), 5.0);

        ammo.controller = BulletController::parametric()
            .with_x("t * 10")
            .with_y("SIN(t)")
            .into();

        ammo.effects.push(damage(DamageType::Energy, 10.0));
    });

    db.component("juh9870:parametric", "eh:weapon").with(|c| {
        c.with_ammunition_id(ammo.id)
            .with_weapon_id(weapon(&db, "juh9870:parametric", 1.0).id)
            .with_layout("1")
    });
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

fn weapon(db: &Database, id: impl DatabaseIdLike<Weapon>, interval: f32) -> DbItem<Weapon> {
    let w = Weapon {
        id: id.into_id(db),
        weapon_class: WeaponClass::Common,
        fire_rate: 1.0 / interval,
        spread: 0.0,
        magazine: 0,
        activation_type: ActivationType::Manual,
        shot_sound: "controls_shot".to_string(),
        charge_sound: "".to_string(),
        shot_effect_prefab: "FlashAdditive".to_string(),
        visual_effect: None,
        effect_size: 1.0,
        control_button_icon: "shot_01".to_string(),
    };
    db.add_item(w)
}
