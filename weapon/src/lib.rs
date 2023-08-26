use bevy::{prelude::*, utils::HashMap};

#[derive(Clone, Copy)]
pub enum PartType {
    GunSight,
    GunGrip,
    GunBody,
    PistolBody,
    GunBarrel,
    SniperBarrel,
    GunStock,
    ShotgunPump,
    Magazine,
    Accessory, // Charms or small decor piece (can be VERY functional still, as in, provides huge gamechanging effects)
    Blade,
    AxeHead,
    Grip, // For melee and bow
    Guard,
    Head,    // Like hammer or mace
    BowLimb, // Will be mirrored top-bottom
    BowString,
    PolearmShaft,
    Shaft,
    Tip, // For polearms or arrows
    ShieldBody,
    Pommel,
    MagicOrnament,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
    Damage,
    AttackSpeed, // Also known as rate-of-fire for projectile weapons
    ReloadSpeed,
    AmmoCount,
    Recoil,
    Accuracy,
    Zoom,
    ADSSpeed,
    BlockChance,
    Defence,
    Lifesteal,
}

type Stats = HashMap<Stat, f64>;

trait Weapon {
    fn get_valid_part_types() -> Vec<PartType>;

    fn get_parts(&self) -> &Vec<Part>;

    fn build_stats(parts: &Vec<Part>) -> Stats {
        let mut stats: Stats = HashMap::new();

        for part in parts {
            for (&stat, &val) in &part.stats {
                stats
                    .entry(stat)
                    .and_modify(|old_val| *old_val += val)
                    .or_insert(val);
            }
        }

        stats
    }
}

#[derive(Component)]
pub struct Part {
    part_type: PartType,
    stats: Stats,
}

pub enum DamageType {
    Pierce,
    Blunt,
    Slash,
    Fire,
    Cold,
    Poison,
    Acid,
    Arcane,
    Void,
}

type DamageInstance = (DamageType, f64, f64);

type Damage = Vec<DamageInstance>;

pub fn calculate_damage(stats: &Stats) -> Damage {
    for (stat, val) in stats {
        match stat {
            Stat::Damage => todo!(),
            Stat::AttackSpeed => todo!(),
            Stat::ReloadSpeed => todo!(),
            Stat::AmmoCount => todo!(),
            Stat::Recoil => todo!(),
            Stat::Accuracy => todo!(),
            Stat::Zoom => todo!(),
            Stat::ADSSpeed => todo!(),
            Stat::BlockChance => todo!(),
            Stat::Defence => todo!(),
            Stat::Lifesteal => todo!(),
        }
    }

    todo!()
}

#[derive(Resource)]
struct Parts(HashMap<PartType, Vec<Part>>);

#[cfg(test)]
mod tests {
    use super::*;

    struct Pistol {
        stats: Stats,
        parts: Vec<Part>,
    }

    impl Weapon for Pistol {
        fn get_valid_part_types() -> Vec<PartType> {
            vec![
                PartType::PistolBody,
                PartType::GunGrip,
                PartType::Magazine,
                PartType::Accessory,
                PartType::GunSight,
            ]
        }

        fn get_parts(&self) -> &Vec<Part> {
            &self.parts
        }
    }

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::bevy::utils::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn it_works() {
        let pistol_body = Part {
            part_type: PartType::PistolBody,
            stats: map! {
                Stat::Damage => 10f64
            },
        };

        let pistol_grip = Part {
            part_type: PartType::GunGrip,
            stats: map! {
                Stat::AttackSpeed => 2f64,
                Stat::ADSSpeed => 2f64,
                Stat::Recoil => 1f64
            },
        };

        let pistol_mag = Part {
            part_type: PartType::Magazine,
            stats: map! {
                Stat::ReloadSpeed => 2f64,
                Stat::Damage => 2f64
            },
        };

        let parts = vec![pistol_body, pistol_grip, pistol_mag];
        let pistol = Pistol {
            stats: Pistol::build_stats(&parts),
            parts,
        };

        assert_eq!(pistol.stats, todo!())
    }
}
