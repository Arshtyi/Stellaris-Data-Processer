pub struct ManualDetail {
    pub cn: &'static str,
    pub en: &'static str,
}

pub struct ModifierInfo {
    pub positive: bool,
    pub mode: &'static str,
}

pub struct LogicInfo {
    pub value: &'static str,
    pub positive: Option<bool>,
}

pub fn get_replace_dict(key: &str) -> Option<&'static str> {
    match key {
        "envoys_add" => Some("MOD_COUNTRY_ENVOYS_ADD"),
        "survey" => Some("SURVEY_ORDER"),
        "is_asteroid" => Some("PLANET_IS_ASTEROID"),

        "all_technology_research_speed" => Some("MOD_COUNTRY_ALL_TECH_RESEARCH_SPEED"),
        "science_ship_survey_speed" => Some("ship_science_survey_speed"),
        "species_leader_exp_gain" => Some("leader_species_exp_gain"),
        "pop_ethics_shift_speed_mult" => Some("MOD_POP_ETHICS_SHIFT_SPEED"),
        "admiral_skill_levels" => Some("leader_admiral_skill_levels"),
        "ruler_skill_levels" => Some("leader_ruler_skill_levels"),
        "governor_skill_levels" => Some("leader_governor_skill_levels"),
        "slave_mineral_output" => Some("pop_slave_mineral_output"),
        "slave_food_output" => Some("pop_slave_food_output"),
        "shipclass_science_ship_disengage_mult" => Some("shipclass_science_ship_evasion_add"),
        "black_hole_pentagruel_research" => Some("black_hole_pantagruel"),
        "adjacency_bonus" => Some("ADJACENCY_BONUS"),
        "physics" => Some("PHYSICS"),
        "society" => Some("SOCIETY"),
        "engineering" => Some("ENGINEERING"),
        // "survey" => Some("调查"),
        "caste_system" => Some("citizenship_caste_system"),
        "personality_type_opportunist" => Some("机会主义者"),
        "personality_type_caste_system" => Some("阶序制度"),
        "personality_type_displacer" => Some("迁移者"),
        "personality_type_multispecies" => Some("多种族者"),
        "personality_type_crisis_fighter" => Some("危机斗士"),
        "personality_type_wants_tribute" => Some("想要贸易"),
        "personality_type_decadent" => Some("颓败"),
        "attack_neutrals" => Some("攻击中立势力"),
        "personality_type_crisis_leader" => Some("危机斗士领导者"),
        "personality_type_custodian" => Some("监护者"),
        "personality_type_enigmatic" => Some("神秘态度"),
        "personality_type_limited" => Some("限制"),
        "personality_type_holy_planets" => Some("重视圣地"),
        "personality_type_demands_clear_borders" => Some("需要边界缓冲区"),
        "personality_type_attack_neutrals" => Some("攻击中立势力"),
        "personality_type_berserker" => Some("狂暴"),
        "democratic" => Some("民主制"),
        "important" => Some("重要"),
        "none" => Some("无"),
        "MOD_SHIP_AUTO_REPAIR_ADD" => Some("每月船体回复值"),
        "requires_technology_glandular_acclimation" => {
            Some("£trigger_no  §R需要£society §Y腺体改造适应§!科技。§!")
        }
        "$NAME_Rampaging_Forest$" => Some("Rampaging Forests"),
        "spawn_chance" => Some("出现概率"),
        "trading_hub" => Some("sm_trading_hub"),
        "shipyard" => Some("sm_shipyard"),
        "anchorage" => Some("sm_anchorage"),
        "AI_STRIKE_CRAFT_1" => Some("AI_STRIKE_CRAFT"),
        "station_small_aura_components" => Some("防御平台光环"),
        "station_medium_aura_components" => Some("防御平台光环"),
        "station_large_aura_components" => Some("防御平台光环"),
        "spaceport" => Some("太空港"),
        "starbase_shipyard" => Some("船坞"),
        "starbase_defenses" => Some("防御"),
        "core" => Some("核心"),
        "is_ironman" => Some("铁人模式"),
        _ => None,
    }
}

pub fn get_manual_dict(key: &str) -> Option<ManualDetail> {
    match key {
        "local_human_species_class" => Some(ManualDetail {
            cn: "玩家种族类型为",
            en: "player species class",
        }),
        "num_fallen_empires_setting" => Some(ManualDetail {
            cn: "设定的失落帝国数",
            en: "number of fallen empires setting",
        }),
        "is_primary_star" => Some(ManualDetail {
            cn: "是星系主恒星",
            en: "is primary star",
        }),
        _ => None,
    }
}

pub fn get_modifier_info(key: &str) -> Option<ModifierInfo> {
    let info = match key {
        // - *
        "empire_size_pops_mult"
        | "country_war_exhaustion_mult"
        | "country_trade_fee"
        | "ship_emergency_ftl_mult"
        | "planet_clear_blocker_time_mult"
        | "species_empire_size_mult"
        | "pop_housing_usage_mult"
        | "country_subject_power_penalty_mult"
        | "empire_size_penalty_mult"
        | "pop_demotion_time_mult"
        | "pop_amenities_usage_mult"
        | "job_criminal_per_crime"
        | "job_deviant_drone_per_crime"
        | "job_corrupt_drone_per_crime"
        | "planet_crime_mult"
        | "planet_pop_assemblers_upkeep_mult"
        | "pop_cat_slave_political_power"
        | "planet_crime_no_happiness_mult"
        | "pop_consumer_goods_mult"
        | "country_border_friction_mult"
        | "planet_unrest_mult"
        | "country_unrest_unhappy_pop_effect_mult"
        | "planet_orbital_bombardment_damage"
        | "pop_food_req_mult"
        | "starbase_shipyard_build_time_mult"
        | "ship_anomaly_fail_risk"
        | "country_unrest_unhappy_slave_effect_mult"
        | "country_integration_cooldown_mult"
        | "mod_distance_to_capital_static_modifier_efficiency_mult"
        | "country_piracy_risk_mult"
        | "ship_emergency_ftl_min_days_mult" => ModifierInfo {
            positive: false,
            mode: "mult",
        },

        // - +
        "planet_crime_add"
        | "job_criminal_add"
        | "job_deviant_drone_add"
        | "job_corrupt_drone_add"
        | "pop_housing_usage_base"
        | "pop_housing_usage_add"
        | "pop_amenities_usage_base"
        | "pop_amenities_usage_no_happiness_base"
        | "planet_unrest_add"
        | "max_food" => ModifierInfo {
            positive: false,
            mode: "add",
        },

        // + +
        "leader_age"
        | "country_admin_cap_add"
        | "country_leader_pool_size"
        | "district_generator_max"
        | "district_mining_max"
        | "district_farming_max"
        | "building_mote_harvesters_max"
        | "building_gas_extractors_max"
        | "building_crystal_mines_max"
        | "building_betharian_power_plant_max"
        | "building_xeno_zoo_max"
        | "pop_political_power"
        | "country_core_sector_system_cap"
        | "max_rivalries"
        | "important"
        | "country_leader_cap"
        | "max_minerals"
        | "max_energy"
        | "pc_nuked_habitability" => ModifierInfo {
            positive: true,
            mode: "add",
        },

        // + *
        "trade_value_mult"
        | "pop_happiness"
        | "pop_citizen_happiness"
        | "species_leader_exp_gain"
        | "ship_disengage_chance_mult"
        | "ship_weapon_range_mult"
        | "ship_evasion_mult"
        | "army_disengage_chance_mult"
        | "army_morale"
        | "army_experience_gain_mult"
        | "edict_length_mult"
        | "country_naval_cap_mult"
        | "ship_anomaly_generation_chance_mult"
        | "pop_environment_tolerance"
        | "pop_growth_from_immigration"
        | "pop_cat_worker_happiness"
        | "pop_cat_slave_happiness"
        | "country_pop_enslaved_mult"
        | "planet_immigration_pull_mult"
        | "country_admin_cap_mult"
        | "diplo_weight_mult"
        | "pop_amenities_usage_no_happiness_mult"
        | "planet_pop_assembly_mult"
        | "empire_size_branch_office_mult"
        | "branch_office_value_mult"
        | "empire_size_systems_mult"
        | "faction_approval"
        | "planet_max_districts_mult"
        | "planet_migration_all_mult"
        | "planet_amenities_mult"
        | "planet_housing_mult"
        | "ship_orbital_bombardment_mult"
        | "planet_building_refund_mult"
        | "planet_amenities_no_happiness_mult"
        | "faction_influence_mult"
        | "rivalry_influence_gain"
        | "country_border_mult"
        | "shipclass_military_station_hit_points_mult"
        | "pop_other_species_happiness"
        | "biological_pop_happiness"
        | "ship_armor_mult"
        | "ship_shield_mult"
        | "pop_owner_happiness"
        | "planet_migration_all_pull"
        | "army_health"
        | "faction_happiness"
        | "planet_migration_xeno_pull"
        | "army_attack_morale_mult"
        | "army_defense_health_mult"
        | "country_trust_growth"
        | "federation_naval_cap_contribution_mult"
        | "shipclass_science_ship_disengage_mult"
        | "country_subject_technology_sharing_mult"
        | "subject_tribute_mult"
        | "country_vassal_naval_capacity_contribution_mult"
        | "pop_eff_wo_slaves"
        | "pop_food_mult"
        | "ship_windup_mult"
        | "ship_winddown_mult"
        | "ship_ftl_jumpdrive_range_mult"
        | "country_unity_produces_mult" => ModifierInfo {
            positive: true,
            mode: "mult",
        },

        "ship_hull_regen_add_perc" | "ship_armor_regen_add_perc" => ModifierInfo {
            positive: true,
            mode: "add_perc",
        },
        "pop_other_species_owner_happiness" => ModifierInfo {
            positive: true,
            mode: "mult",
        },

        _ => {
            // Fallback logic (preprocess_pdx_dict)
            let lower_key = key.to_lowercase();
            // district_X_max or building_X_max
            if (lower_key.starts_with("district_") && lower_key.ends_with("_max"))
                || (lower_key.starts_with("building_") && lower_key.ends_with("_max"))
            {
                return Some(ModifierInfo {
                    positive: true,
                    mode: "add",
                });
            }

            if lower_key.contains("_add")
                || lower_key.contains("level")
                || lower_key.contains("add_static")
            {
                return Some(ModifierInfo {
                    positive: true,
                    mode: "add",
                });
            }
            if lower_key.contains("reduction")
                || lower_key.contains("cost")
                || lower_key.contains("upkeep")
            {
                return Some(ModifierInfo {
                    positive: false,
                    mode: "mult",
                });
            }
            if lower_key.contains("speed")
                || lower_key.contains("resource")
                || lower_key.contains("fire_rate_mult")
                || lower_key.contains("hull_mult")
                || lower_key.contains("damage")
                || lower_key.contains("attract")
                || lower_key.contains("output")
                || lower_key.contains("_habitability")
                || lower_key.contains("produces_mult")
                || (lower_key.contains("job_") && lower_key.contains("_per_pop"))
            {
                return Some(ModifierInfo {
                    positive: true,
                    mode: "mult",
                });
            }

            return None;
        }
    };
    Some(info)
}

pub fn get_logic_info(key: &str) -> Option<LogicInfo> {
    match key {
        "AND" => Some(LogicInfo {
            value: "AND_TRIGGER_STARTS",
            positive: Some(true),
        }),
        "OR" => Some(LogicInfo {
            value: "OR_TRIGGER_STARTS",
            positive: Some(true),
        }),
        "NOR" | "not_OR" => Some(LogicInfo {
            value: "NOR_TRIGGER_STARTS",
            positive: Some(false),
        }),
        "NAND" => Some(LogicInfo {
            value: "NAND_TRIGGER_STARTS",
            positive: Some(false),
        }),
        "has_civic" => Some(LogicInfo {
            value: "TRIGGER_HAS_CIVIC",
            positive: Some(true),
        }),

        "if" => Some(LogicInfo {
            value: "如果",
            positive: None,
        }),
        "else" => Some(LogicInfo {
            value: "否则",
            positive: None,
        }),
        "else_if" => Some(LogicInfo {
            value: "或者如果",
            positive: None,
        }),
        "hidden_effect" | "hidden_trigger" => Some(LogicInfo {
            value: "隐藏效果",
            positive: None,
        }),
        "limit" => Some(LogicInfo {
            value: "满足以下条件时：",
            positive: None,
        }),
        "potential" => Some(LogicInfo {
            value: "基础要求",
            positive: None,
        }),
        "custom_tooltip" => Some(LogicInfo {
            value: "",
            positive: None,
        }),
        "value" => Some(LogicInfo {
            value: "基础值 = ",
            positive: None,
        }),
        "factor" => Some(LogicInfo {
            value: "系数 × ",
            positive: None,
        }),
        "weight" => Some(LogicInfo {
            value: "权重 = ",
            positive: None,
        }),
        "random_weight" => Some(LogicInfo {
            value: "随机权重",
            positive: None,
        }),
        "break" => Some(LogicInfo {
            value: "分支结束",
            positive: None,
        }),

        "is_robotic" | "is_robot_pop" => Some(LogicInfo {
            value: "是机械人口",
            positive: None,
        }),
        "has_living_standard" => Some(LogicInfo {
            value: "生活标准为",
            positive: Some(true),
        }),
        "has_election_type" => Some(LogicInfo {
            value: "选举方式为",
            positive: None,
        }),
        "is_in_federation_with" => Some(LogicInfo {
            value: "IS_IN_FEDERATION_WITH_TOOLTIP",
            positive: Some(true),
        }),
        "is_non_sapient_robot" => Some(LogicInfo {
            value: "是非开智机械",
            positive: Some(true),
        }),
        "is_hostile_to" => Some(LogicInfo {
            value: "与$has_a$敌对",
            positive: Some(true),
        }),
        "years_of_peace" => Some(LogicInfo {
            value: "和平年数",
            positive: None,
        }),
        "root" => Some(LogicInfo {
            value: "己方",
            positive: None,
        }),
        "who" => Some(LogicInfo {
            value: "主体",
            positive: None,
        }),
        "from" => Some(LogicInfo {
            value: "对方",
            positive: Some(true),
        }),
        "delay" => Some(LogicInfo {
            value: "延迟天数",
            positive: None,
        }),
        "overlord" => Some(LogicInfo {
            value: "宗主",
            positive: None,
        }),
        "is_pop_faction_type" => Some(LogicInfo {
            value: "派系是：",
            positive: Some(true),
        }),
        "mid_game_years_passed" => Some(LogicInfo {
            value: "游戏中期已过年数",
            positive: None,
        }),
        "years_passed" => Some(LogicInfo {
            value: "年数",
            positive: Some(true),
        }),
        "not_from" => Some(LogicInfo {
            value: "没有",
            positive: Some(false),
        }),
        "civics" => Some(LogicInfo {
            value: "GOVERNMENT_CIVICS",
            positive: None,
        }),
        "authority" | "GOVERNMENT_AUTHORITY" => Some(LogicInfo {
            value: "GOVERNMENT_AUTHORITY",
            positive: None,
        }),
        "not_value" => Some(LogicInfo {
            value: "基础值≠ ",
            positive: None,
        }),
        "country_type" => Some(LogicInfo {
            value: "IS_COUNTRY_TYPE",
            positive: None,
        }),
        "has_research_agreement" => Some(LogicInfo {
            value: "拥有研究协议",
            positive: Some(true),
        }),
        "random_owned_pop" => Some(LogicInfo {
            value: "effect_random_owned_pop",
            positive: None,
        }),
        "add_deposit" => Some(LogicInfo {
            value: "在目标星球上添加$has_a$",
            positive: None,
        }),
        "add_district" => Some(LogicInfo {
            value: "ADD_DISTRICT_EFFECT",
            positive: None,
        }),
        "has_country_flag" => Some(LogicInfo {
            value: "有国家标识",
            positive: Some(true),
        }),
        "tech_unlocked_ratio" => Some(LogicInfo {
            value: "解锁科技的比率",
            positive: None,
        }),
        "ratio" => Some(LogicInfo {
            value: "比率",
            positive: None,
        }),
        "set_subject_of" => Some(LogicInfo {
            value: "附庸国设定为",
            positive: None,
        }),
        "subject_type" => Some(LogicInfo {
            value: "附庸类型",
            positive: None,
        }),
        "join_war" => Some(LogicInfo {
            value: "加入战争",
            positive: None,
        }),
        "shift_ethic" => Some(LogicInfo {
            value: "转变思潮",
            positive: None,
        }),
        "count_owned_pops" => Some(LogicInfo {
            value: "拥有的人口：",
            positive: None,
        }),
        "count" => Some(LogicInfo {
            value: "数量",
            positive: Some(true),
        }),
        "valid_objects" => Some(LogicInfo {
            value: "符合条件的物体",
            positive: None,
        }),
        "not_exists" => Some(LogicInfo {
            value: "没有",
            positive: Some(false),
        }),
        "is_event_leader" | "leader" => Some(LogicInfo {
            value: "事件领袖",
            positive: None,
        }),
        "pop_faction_event" => Some(LogicInfo {
            value: "触发派系事件",
            positive: None,
        }),
        "id" => Some(LogicInfo {
            value: "id",
            positive: None,
        }),

        "has_valid_civic" => Some(LogicInfo {
            value: "TRIGGER_HAS_VALID_CIVIC",
            positive: Some(true),
        }),
        "not_has_valid_civic" => Some(LogicInfo {
            value: "TRIGGER_HAS_NOT_VALID_CIVIC",
            positive: Some(false),
        }),
        "is_same_value" => Some(LogicInfo {
            value: "与$has_a$相同",
            positive: Some(true),
        }),
        "not_is_same_value" => Some(LogicInfo {
            value: "与$has_a$不相同",
            positive: Some(false),
        }),
        "is_same_species" => Some(LogicInfo {
            value: "IS_SAME_SPECIES",
            positive: Some(true),
        }),
        "not_is_same_species" => Some(LogicInfo {
            value: "IS_NOT_SAME_SPECIES",
            positive: Some(false),
        }),
        "has_trait" => Some(LogicInfo {
            value: "HAS_TRAIT",
            positive: Some(true),
        }),
        "not_has_trait" => Some(LogicInfo {
            value: "DOES_NOT_HAVE_TRAIT",
            positive: Some(false),
        }),
        "is_controlled_by" => Some(LogicInfo {
            value: "IS_CONTROLLED_BY",
            positive: Some(true),
        }),
        "not_is_controlled_by" => Some(LogicInfo {
            value: "IS_NOT_CONTROLLED_BY",
            positive: Some(false),
        }),
        "has_policy_flag" => Some(LogicInfo {
            value: "拥有政策标识",
            positive: Some(true),
        }),
        "not_has_policy_flag" => Some(LogicInfo {
            value: "没有政策标识",
            positive: Some(false),
        }),
        "has_origin" => Some(LogicInfo {
            value: "TRIGGER_HAS_ORIGIN",
            positive: Some(true),
        }),
        "not_has_origin" => Some(LogicInfo {
            value: "TRIGGER_NOT_HAS_ORIGIN",
            positive: Some(false),
        }),
        "host_has_dlc" => Some(LogicInfo {
            value: "主机拥有DLC：",
            positive: Some(true),
        }),
        "not_host_has_dlc" => Some(LogicInfo {
            value: "主机没有DLC：",
            positive: Some(false),
        }),
        "has_global_flag" => Some(LogicInfo {
            value: "HAS_GLOBAL_FLAG",
            positive: Some(true),
        }),
        "not_has_global_flag" => Some(LogicInfo {
            value: "HAS_NOT_GLOBAL_FLAG",
            positive: Some(false),
        }),
        "is_domineering_to" => Some(LogicInfo {
            value: "IS_DOMINEERING_TO_TOOLTIP",
            positive: Some(true),
        }),
        "not_is_domineering_to" => Some(LogicInfo {
            value: "IS_NOT_DOMINEERING_TO_TOOLTIP",
            positive: Some(false),
        }),
        "has_ethic" => Some(LogicInfo {
            value: "HAS_ETHIC",
            positive: Some(true),
        }),
        "not_has_ethic" => Some(LogicInfo {
            value: "DOES_NOT_HAVE_ETHIC",
            positive: Some(false),
        }),
        "num_free_districts" => Some(LogicInfo {
            value: "HAS_NUM_FREE_DISTRICTS",
            positive: Some(true),
        }),
        "not_num_free_districts" => Some(LogicInfo {
            value: "HAS_NOT_NUM_FREE_DISTRICTS",
            positive: Some(false),
        }),
        "has_star_flag" => Some(LogicInfo {
            value: "有恒星标识",
            positive: Some(true),
        }),
        "not_has_star_flag" => Some(LogicInfo {
            value: "没有恒星标识",
            positive: Some(false),
        }),
        "leader_class" => Some(LogicInfo {
            value: "领袖是",
            positive: Some(true),
        }),
        "not_leader_class" => Some(LogicInfo {
            value: "领袖不是",
            positive: Some(false),
        }),
        "is_star" => Some(LogicInfo {
            value: "PLANET_IS_STAR",
            positive: Some(true),
        }),
        "not_is_star" => Some(LogicInfo {
            value: "PLANET_IS_NOT_STAR",
            positive: Some(false),
        }),
        "has_ascension_perk" => Some(LogicInfo {
            value: "TRIGGER_HAS_ASCENSION_PERK",
            positive: Some(true),
        }),
        "not_has_ascension_perk" => Some(LogicInfo {
            value: "TRIGGER_NOT_HAS_ASCENSION_PERK",
            positive: Some(false),
        }),
        "has_authority" => Some(LogicInfo {
            value: "有$has_a$政体",
            positive: Some(true),
        }),
        "not_has_authority" => Some(LogicInfo {
            value: "没有$has_a$政体",
            positive: Some(false),
        }),
        "is_country_type" => Some(LogicInfo {
            value: "IS_COUNTRY_TYPE",
            positive: Some(true),
        }),
        "not_is_country_type" => Some(LogicInfo {
            value: "IS_NOT_COUNTRY_TYPE",
            positive: Some(false),
        }),

        // DLC
        "Leviathans Story Pack" => Some(LogicInfo {
            value: "利维坦",
            positive: Some(true),
        }),
        "Utopia" => Some(LogicInfo {
            value: "乌托邦",
            positive: Some(true),
        }),
        "Synthetic Dawn Story Pack" => Some(LogicInfo {
            value: "合成人黎明",
            positive: Some(true),
        }),
        "Apocalypse" => Some(LogicInfo {
            value: "启示录",
            positive: Some(true),
        }),
        "Distant Stars Story Pack" => Some(LogicInfo {
            value: "遥远星系",
            positive: Some(true),
        }),
        "Megacorp" => Some(LogicInfo {
            value: "寰宇企业",
            positive: Some(true),
        }),
        "Ancient Relics Story Pack" => Some(LogicInfo {
            value: "远古遗迹",
            positive: Some(true),
        }),
        "Lithoids Species Pack" => Some(LogicInfo {
            value: "石质种族",
            positive: Some(true),
        }),
        "Federations" => Some(LogicInfo {
            value: "联邦",
            positive: Some(true),
        }),

        "is_preferred_weapons" => Some(LogicInfo {
            value: "IS_PREFERRED_WEAPONS_TRIGGER",
            positive: Some(true),
        }),
        "not_is_preferred_weapons" => Some(LogicInfo {
            value: "IS_NOT_PREFERRED_WEAPONS_TRIGGER",
            positive: Some(false),
        }),
        "not_has_civic" => Some(LogicInfo {
            value: "TRIGGER_NOT_HAS_CIVIC",
            positive: Some(true),
        }),
        "allows_slavery" => Some(LogicInfo {
            value: "ALLOWS_SLAVERY",
            positive: Some(true),
        }),
        "modifier" => Some(LogicInfo {
            value: "修正：",
            positive: Some(true),
        }),
        "weight_modifier" => Some(LogicInfo {
            value: "权重修正",
            positive: Some(true),
        }),
        "exists" => Some(LogicInfo {
            value: "$啥玩意$存在",
            positive: Some(true),
        }),
        "pop_faction" => Some(LogicInfo {
            value: "POP_FACTION",
            positive: Some(true),
        }),
        "slavery_not_allowed" => Some(LogicInfo {
            value: "禁止奴隶制",
            positive: Some(true),
        }),
        "num_communications" => Some(LogicInfo {
            value: "建立通讯数量",
            positive: Some(true),
        }),
        "has_government" => Some(LogicInfo {
            value: "有$has_a$政体",
            positive: Some(true),
        }),
        "not_has_government" => Some(LogicInfo {
            value: "没有$has_a$政体",
            positive: Some(false),
        }),
        "is_planet_class" => Some(LogicInfo {
            value: "星球类型是",
            positive: Some(true),
        }),
        "not_is_planet_class" => Some(LogicInfo {
            value: "星球类型不是",
            positive: Some(false),
        }),
        "num_modifiers" => Some(LogicInfo {
            value: "修正数量",
            positive: None,
        }),
        "spawn_chance" => Some(LogicInfo {
            value: "出现概率",
            positive: None,
        }),
        "has_planet_modifier" => Some(LogicInfo {
            value: "有星球修正",
            positive: Some(true),
        }),
        "add" => Some(LogicInfo {
            value: "增加",
            positive: Some(true),
        }),
        "num_moons" => Some(LogicInfo {
            value: "卫星数量",
            positive: None,
        }),
        "has_moon" => Some(LogicInfo {
            value: "拥有卫星：",
            positive: None,
        }),
        "no" => Some(LogicInfo {
            value: "否",
            positive: None,
        }),
        "yes" => Some(LogicInfo {
            value: "是",
            positive: None,
        }),
        "is_moon" => Some(LogicInfo {
            value: "是卫星",
            positive: None,
        }),
        "planet_size" => Some(LogicInfo {
            value: "星球大小",
            positive: None,
        }),
        "ethics" => Some(LogicInfo {
            value: "ETHICS",
            positive: None,
        }),
        "base" => Some(LogicInfo {
            value: "基础值",
            positive: None,
        }),
        "default" => Some(LogicInfo {
            value: "普通帝国",
            positive: None,
        }),
        "primitive" => Some(LogicInfo {
            value: "原始文明",
            positive: None,
        }),
        "enclave" => Some(LogicInfo {
            value: "城邦",
            positive: None,
        }),
        "sentinels" => Some(LogicInfo {
            value: "哨兵",
            positive: None,
        }),
        "ai_empire" => Some(LogicInfo {
            value: "肃正协议",
            positive: None,
        }),
        "dormant_marauders" => Some(LogicInfo {
            value: "劫掠者",
            positive: None,
        }),
        "awakened_marauders" => Some(LogicInfo {
            value: "复兴劫掠者",
            positive: None,
        }),
        "drop_weight" => Some(LogicInfo {
            value: "地块资源权重：",
            positive: None,
        }),
        "orbital_weight" => Some(LogicInfo {
            value: "轨道资源权重：",
            positive: None,
        }),
        "planet" => Some(LogicInfo {
            value: "星球",
            positive: None,
        }),
        "has_owner" => Some(LogicInfo {
            value: "已被殖民：",
            positive: Some(true),
        }),
        "has_strategic_resource" => Some(LogicInfo {
            value: "有战略资源：",
            positive: Some(true),
        }),
        "is_inside_nebula" => Some(LogicInfo {
            value: "在星云内：",
            positive: Some(true),
        }),
        "not_is_in_cluster" => Some(LogicInfo {
            value: "不在资源聚集区",
            positive: Some(false),
        }),
        "is_in_cluster" => Some(LogicInfo {
            value: "在资源聚集区",
            positive: Some(true),
        }),
        "not_has_country_flag" => Some(LogicInfo {
            value: "没有国家标识",
            positive: Some(false),
        }),
        "has_deposit" => Some(LogicInfo {
            value: "有资源点",
            positive: Some(true),
        }),
        "solar_system" => Some(LogicInfo {
            value: "恒星系",
            positive: None,
        }),
        "is_star_class" => Some(LogicInfo {
            value: "恒星类型",
            positive: Some(true),
        }),
        "has_swapped_tradition" => Some(LogicInfo {
            value: "有$has_a$传统",
            positive: Some(true),
        }),
        "not_has_swapped_tradition" => Some(LogicInfo {
            value: "没有$has_a$传统",
            positive: Some(false),
        }),
        "has_non_swapped_tradition" => Some(LogicInfo {
            value: "有$has_a$传统",
            positive: Some(true),
        }),
        "not_has_non_swapped_tradition" => Some(LogicInfo {
            value: "没有$has_a$传统",
            positive: Some(false),
        }),
        "owner" => Some(LogicInfo {
            value: "拥有者：",
            positive: Some(true),
        }),
        "not_owner" => Some(LogicInfo {
            value: "拥有者没有：",
            positive: Some(false),
        }),
        "is_capital" => Some(LogicInfo {
            value: "是首都星球：",
            positive: Some(true),
        }),
        "has_building" => Some(LogicInfo {
            value: "拥有建筑：",
            positive: Some(true),
        }),
        "not_has_building" => Some(LogicInfo {
            value: "没有建筑：",
            positive: Some(false),
        }),
        "planet.owner" => Some(LogicInfo {
            value: "星球拥有者",
            positive: None,
        }),
        "check_variable" => Some(LogicInfo {
            value: "检查变量：",
            positive: None,
        }),
        "which" => Some(LogicInfo {
            value: "",
            positive: Some(true),
        }),
        "ai_weight" => Some(LogicInfo {
            value: "AI权重",
            positive: None,
        }),
        "ai_allow" => Some(LogicInfo {
            value: "AI建造条件",
            positive: None,
        }),
        "allow" => Some(LogicInfo {
            value: "要求",
            positive: None,
        }),
        "amount" => Some(LogicInfo {
            value: "数量",
            positive: None,
        }),
        "tile" => Some(LogicInfo {
            value: "地块",
            positive: None,
        }),
        "has_resource" => Some(LogicInfo {
            value: "拥有资源：",
            positive: Some(true),
        }),
        "type" => Some(LogicInfo {
            value: "类型",
            positive: None,
        }),
        "sector_controlled" => Some(LogicInfo {
            value: "由星区掌控：",
            positive: Some(true),
        }),
        "has_technology" => Some(LogicInfo {
            value: "拥有$has_a$科技",
            positive: Some(true),
        }),
        "not_has_technology" => Some(LogicInfo {
            value: "没有$has_a$科技",
            positive: Some(false),
        }),
        "not_planet" => Some(LogicInfo {
            value: "星球没有",
            positive: Some(false),
        }),
        "is_enslaved" => Some(LogicInfo {
            value: "被奴役",
            positive: Some(true),
        }),
        "count_pops" => Some(LogicInfo {
            value: "满足条件人口：",
            positive: Some(true),
        }),
        "count_tile" => Some(LogicInfo {
            value: "满足条件地块：",
            positive: Some(true),
        }),
        "unrest" => Some(LogicInfo {
            value: "UNREST",
            positive: Some(true),
        }),
        "destroy_if" => Some(LogicInfo {
            value: "满足以下条件强制拆除",
            positive: None,
        }),
        "show_tech_unlock_if" => Some(LogicInfo {
            value: "显示其科技所需条件",
            positive: None,
        }),
        "has_megastructure" => Some(LogicInfo {
            value: "拥有巨型建筑：",
            positive: Some(true),
        }),
        "active" => Some(LogicInfo {
            value: "",
            positive: None,
        }),
        "has_grown_pop" => Some(LogicInfo {
            value: "有已成长人口：",
            positive: Some(true),
        }),
        "pop" => Some(LogicInfo {
            value: "人口",
            positive: None,
        }),
        "resources" => Some(LogicInfo {
            value: "额外产出：",
            positive: Some(true),
        }),
        "research_leader" => Some(LogicInfo {
            value: "科学领袖：",
            positive: None,
        }),
        "not_research_leader" => Some(LogicInfo {
            value: "科学领袖未达到：",
            positive: None,
        }),
        "has_tradition" => Some(LogicInfo {
            value: "拥有$has_a$传统",
            positive: Some(true),
        }),
        "has_level" => Some(LogicInfo {
            value: "等级",
            positive: Some(true),
        }),
        "has_modifier" => Some(LogicInfo {
            value: "有修正",
            positive: Some(true),
        }),
        "not_has_modifier" => Some(LogicInfo {
            value: "没有修正",
            positive: Some(false),
        }),
        "num_owned_planets" => Some(LogicInfo {
            value: "拥有行星数量",
            positive: None,
        }),
        "has_ai_personality" | "has_ai_personality_behaviour" => Some(LogicInfo {
            value: "拥有$has_a$性格",
            positive: Some(true),
        }),
        "any_relation" => Some(LogicInfo {
            value: "任何相关国家满足：",
            positive: Some(true),
        }),
        "not_any_relation" => Some(LogicInfo {
            value: "任何相关国家都不满足：",
            positive: Some(false),
        }),
        "has_communications" => Some(LogicInfo {
            value: "与$has_a$有通讯",
            positive: Some(true),
        }),
        "ROOT" => Some(LogicInfo {
            value: "我们",
            positive: None,
        }),
        "any_owned_pop" | "any_owned_pops" => Some(LogicInfo {
            value: "任意拥有的人口：",
            positive: None,
        }),
        "not_any_owned_pop" => Some(LogicInfo {
            value: "任意拥有的人口都不满足：",
            positive: Some(false),
        }),
        "any_system_within_border" => Some(LogicInfo {
            value: "境内星系：",
            positive: None,
        }),
        "election_candidates" => Some(LogicInfo {
            value: "参选权重：",
            positive: None,
        }),
        "has_federation" => Some(LogicInfo {
            value: "存在联邦：",
            positive: None,
        }),
        "not_any_subject" => Some(LogicInfo {
            value: "任意附庸都不满足",
            positive: Some(false),
        }),
        "is_subject_type" => Some(LogicInfo {
            value: "附庸类型为",
            positive: Some(true),
        }),
        "not_is_subject_type" => Some(LogicInfo {
            value: "附庸类型不是",
            positive: Some(false),
        }),
        "is_colonizable" => Some(LogicInfo {
            value: "是可殖民星球：",
            positive: None,
        }),
        "not_has_ai_personality_behaviour" => Some(LogicInfo {
            value: "没有$PERSONALITY|Y$行为",
            positive: Some(false),
        }),

        "is_xenophile" => Some(LogicInfo {
            value: "是亲外主义：",
            positive: None,
        }),
        "is_xenophobe" => Some(LogicInfo {
            value: "是排外主义：",
            positive: None,
        }),
        "is_pacifist" => Some(LogicInfo {
            value: "是和平主义：",
            positive: None,
        }),
        "is_militarist" => Some(LogicInfo {
            value: "是军国主义：",
            positive: None,
        }),
        "is_materialist" => Some(LogicInfo {
            value: "是唯物主义：",
            positive: None,
        }),
        "is_spiritualist" => Some(LogicInfo {
            value: "是唯心主义：",
            positive: None,
        }),
        "is_egalitarian" => Some(LogicInfo {
            value: "是平等主义：",
            positive: None,
        }),
        "is_authoritarian" => Some(LogicInfo {
            value: "是威权主义：",
            positive: None,
        }),
        "is_colony" => Some(LogicInfo {
            value: "是殖民地：",
            positive: None,
        }),

        "any_neighbor_country" => Some(LogicInfo {
            value: "任意邻国",
            positive: None,
        }),
        "any_country" => Some(LogicInfo {
            value: "任意国家",
            positive: None,
        }),
        "not_any_country" => Some(LogicInfo {
            value: "任意国家都不满足",
            positive: Some(false),
        }),
        "num_ascension_perks" => Some(LogicInfo {
            value: "飞升数量",
            positive: None,
        }),
        "is_mechanical_empire" => Some(LogicInfo {
            value: "是机械帝国：",
            positive: None,
        }),
        "is_cyborg_empire" => Some(LogicInfo {
            value: "是合成人帝国：",
            positive: None,
        }),
        "condition" => Some(LogicInfo {
            value: "条件",
            positive: None,
        }),
        "is_owned_by" => Some(LogicInfo {
            value: "是$has_a$所有",
            positive: None,
        }),
        "area" => Some(LogicInfo {
            value: "领域：",
            positive: None,
        }),
        "has_orbital_bombardment" => Some(LogicInfo {
            value: "轨道轰炸中：",
            positive: None,
        }),
        "num_pops" => Some(LogicInfo {
            value: "人口数",
            positive: None,
        }),
        "has_comms_with_alien_empire" => Some(LogicInfo {
            value: "与外星帝国有过接触",
            positive: None,
        }),
        "has_comms_with_alien_civilization" => Some(LogicInfo {
            value: "与外星文明有过接触",
            positive: None,
        }),
        "habitable_structure" => Some(LogicInfo {
            value: "是栖息地：",
            positive: None,
        }),
        "not_has_planet_flag" => Some(LogicInfo {
            value: "没有行星标识",
            positive: Some(false),
        }),
        "starbase" => Some(LogicInfo {
            value: "恒星基地",
            positive: None,
        }),
        "set_planet_flag" => Some(LogicInfo {
            value: "添加行星标识",
            positive: None,
        }),
        "set_star_flag" => Some(LogicInfo {
            value: "添加恒星标识",
            positive: None,
        }),
        "set_country_flag" => Some(LogicInfo {
            value: "添加国家标识",
            positive: None,
        }),
        "any_neighbor_system" => Some(LogicInfo {
            value: "任意相邻星系",
            positive: None,
        }),
        "every_system_planet" => Some(LogicInfo {
            value: "星系内所有星球",
            positive: None,
        }),
        "remove_planet" => Some(LogicInfo {
            value: "移除星球",
            positive: None,
        }),
        "asteroids_distance" => Some(LogicInfo {
            value: "小行星带半径",
            positive: None,
        }),
        "remove_megastructure" => Some(LogicInfo {
            value: "移除巨型建筑",
            positive: None,
        }),
        "spawn_megastructure" => Some(LogicInfo {
            value: "生成该巨型建筑的巨型建筑",
            positive: None,
        }),
        "name" | "set_name" => Some(LogicInfo {
            value: "名称：",
            positive: None,
        }),
        "orbit_angle" => Some(LogicInfo {
            value: "轨道角度",
            positive: None,
        }),
        "orbit_distance" => Some(LogicInfo {
            value: "轨道半径",
            positive: None,
        }),
        "location" => Some(LogicInfo {
            value: "基准位置",
            positive: None,
        }),
        "change_pc" => Some(LogicInfo {
            value: "星球类型更换为",
            positive: None,
        }),
        "every_system_ambient_object" => Some(LogicInfo {
            value: "星系内所有环境物体",
            positive: None,
        }),
        "destroy_ambient_object" => Some(LogicInfo {
            value: "摧毁环境物体",
            positive: None,
        }),
        "country_event" => Some(LogicInfo {
            value: "触发国家事件",
            positive: None,
        }),
        "planet_possible" => Some(LogicInfo {
            value: "行星满足要求：",
            positive: None,
        }),
        "spawn_planet" => Some(LogicInfo {
            value: "创建星球：",
            positive: None,
        }),
        "orbit_angle_offset" => Some(LogicInfo {
            value: "轨道角度偏移：",
            positive: None,
        }),
        "init_effect" => Some(LogicInfo {
            value: "初始设定：",
            positive: None,
        }),
        "set_planet_entity" => Some(LogicInfo {
            value: "模型设定：",
            positive: None,
        }),
        "entity" => Some(LogicInfo {
            value: "模型：",
            positive: None,
        }),
        "surveyed" => Some(LogicInfo {
            value: "调查情况：",
            positive: None,
        }),
        "set_surveyed" => Some(LogicInfo {
            value: "已调查：",
            positive: None,
        }),
        "set_all_comms_surveyed" => Some(LogicInfo {
            value: "所有已通讯国家均调查过：",
            positive: None,
        }),
        "class" => Some(LogicInfo {
            value: "类型",
            positive: None,
        }),
        "any_tile" => Some(LogicInfo {
            value: "任意地块",
            positive: None,
        }),
        "has_blocker" => Some(LogicInfo {
            value: "有地块障碍",
            positive: None,
        }),
        "random_tile" => Some(LogicInfo {
            value: "随机地块",
            positive: None,
        }),
        "remove_blocker" => Some(LogicInfo {
            value: "移除地块障碍",
            positive: None,
        }),
        "save_event_target_as" => Some(LogicInfo {
            value: "设置为$has_a$事件目标",
            positive: None,
        }),
        "trigger_megastructure_icon" => Some(LogicInfo {
            value: "设置巨型建筑图标",
            positive: None,
        }),
        "fromfrom.planet" => Some(LogicInfo {
            value: "目标行星",
            positive: None,
        }),
        "event_target" => Some(LogicInfo {
            value: "事件目标",
            positive: None,
        }),
        "while" => Some(LogicInfo {
            value: "循环",
            positive: None,
        }),
        "remove_star_flag" => Some(LogicInfo {
            value: "移除恒星标识",
            positive: None,
        }),
        "this" => Some(LogicInfo {
            value: "",
            positive: None,
        }),
        "activate_gateway" => Some(LogicInfo {
            value: "激活星门",
            positive: None,
        }),
        "num_active_gateways" => Some(LogicInfo {
            value: "已激活星门数",
            positive: None,
        }),
        "random_megastructure" => Some(LogicInfo {
            value: "随机巨型建筑",
            positive: None,
        }),
        "is_megastructure_type" => Some(LogicInfo {
            value: "巨型建筑类型为",
            positive: None,
        }),
        "upgrade_megastructure_to" => Some(LogicInfo {
            value: "升级为巨型建筑",
            positive: None,
        }),
        "finish_upgrade" => Some(LogicInfo {
            value: "完成升级",
            positive: None,
        }),
        "orbit_location" => Some(LogicInfo {
            value: "轨道位置",
            positive: None,
        }),
        "orbit_distance_offset" => Some(LogicInfo {
            value: "轨道距离偏移量",
            positive: None,
        }),
        "size" => Some(LogicInfo {
            value: "星球",
            positive: None,
        }),
        "has_ring" => Some(LogicInfo {
            value: "是否有环",
            positive: None,
        }),
        "colonizeable_planet" => Some(LogicInfo {
            value: "是可宜居行星",
            positive: None,
        }),
        "copy_orbital_tile" => Some(LogicInfo {
            value: "复制$has_a$的轨道资源地块",
            positive: None,
        }),
        "is_subject" => Some(LogicInfo {
            value: "是附庸国",
            positive: None,
        }),
        "is_berserk_fallen_machine_empire" => Some(LogicInfo {
            value: "是§H失常的监护§!",
            positive: None,
        }),
        "has_claim" => Some(LogicInfo {
            value: "拥有宣称：",
            positive: None,
        }),
        "FROM" => Some(LogicInfo {
            value: "防御方",
            positive: None,
        }),
        "has_total_war_cb" => Some(LogicInfo {
            value: "拥有§H灭绝战争§!的宣战借口",
            positive: None,
        }),
        "is_rival" => Some(LogicInfo {
            value: "是$has_a$的宿敌",
            positive: None,
        }),
        "is_neighbor_of" => Some(LogicInfo {
            value: "与$has_a$相邻",
            positive: None,
        }),
        "any_ship" => Some(LogicInfo {
            value: "任意舰船",
            positive: None,
        }),
        "not_any_ship" => Some(LogicInfo {
            value: "任意舰船都不是",
            positive: Some(false),
        }),
        "is_ship_size" => Some(LogicInfo {
            value: "舰船类型是",
            positive: None,
        }),
        "has_opinion_modifier" => Some(LogicInfo {
            value: "拥有好感度修正",
            positive: None,
        }),
        "is_at_war" => Some(LogicInfo {
            value: "处于战争状态",
            positive: None,
        }),
        "has_met_primitives" => Some(LogicInfo {
            value: "已遇见过原始文明",
            positive: None,
        }),
        "can_set_ai_policy" => Some(LogicInfo {
            value: "可以设置人工智能政策",
            positive: None,
        }),
        "can_set_robot_policy" => Some(LogicInfo {
            value: "可以设置机器人政策",
            positive: None,
        }),
        "any_planet_within_border" => Some(LogicInfo {
            value: "境内任意行星",
            positive: None,
        }),
        "any_pop" => Some(LogicInfo {
            value: "任意人口",
            positive: None,
        }),
        "is_sapient" => Some(LogicInfo {
            value: "是已开智人口",
            positive: None,
        }),
        "has_encountered_other_species" => Some(LogicInfo {
            value: "已遇见过其他种族",
            positive: None,
        }),
        "has_valid_ai_personality" => Some(LogicInfo {
            value: "有有效的AI性格",
            positive: None,
        }),
        "multispecies" => Some(LogicInfo {
            value: "给予外星物种权利",
            positive: None,
        }),
        "attack_neutrals" => Some(LogicInfo {
            value: "攻击中立势力",
            positive: None,
        }),
        "is_ai" => Some(LogicInfo {
            value: "是AI",
            positive: None,
        }),
        "check_casus_belli_valid" => Some(LogicInfo {
            value: "检查以下宣战借口的可用性",
            positive: None,
        }),
        "is_shackled_robot" => Some(LogicInfo {
            value: "被奴役的机械",
            positive: None,
        }),
        "is_being_purged" => Some(LogicInfo {
            value: "正在被净化的人口",
            positive: None,
        }),
        "has_culture_shock" => Some(LogicInfo {
            value: "被文化冲击的人口",
            positive: None,
        }),
        "remove_modifier" => Some(LogicInfo {
            value: "移除修正",
            positive: None,
        }),
        "owner_species" => Some(LogicInfo {
            value: "主体种族",
            positive: None,
        }),
        "species" => Some(LogicInfo {
            value: "种族",
            positive: None,
        }),
        "root.owner" => Some(LogicInfo {
            value: "所属国家",
            positive: None,
        }),
        "has_any_tradition_unlocked" => Some(LogicInfo {
            value: "已解锁任意传统",
            positive: None,
        }),
        "prev" => Some(LogicInfo {
            value: "前者",
            positive: None,
        }),
        "count_neighbor_country" => Some(LogicInfo {
            value: "邻国数量",
            positive: None,
        }),
        "is_overlord" => Some(LogicInfo {
            value: "宗主国",
            positive: None,
        }),
        "guardian" => Some(LogicInfo {
            value: "守护者",
            positive: None,
        }),
        "guardian_dragon" => Some(LogicInfo {
            value: "以太巨龙",
            positive: None,
        }),
        "guardian_stellarite" => Some(LogicInfo {
            value: "噬星者",
            positive: None,
        }),
        "guardian_wraith" => Some(LogicInfo {
            value: "幽魂",
            positive: None,
        }),
        "guardian_hiver" => Some(LogicInfo {
            value: "蜂巢小行星",
            positive: None,
        }),
        "guardian_horror" => Some(LogicInfo {
            value: "位面之魇",
            positive: None,
        }),
        "guardian_fortress" => Some(LogicInfo {
            value: "神秘堡垒",
            positive: None,
        }),
        "guardian_dreadnought" => Some(LogicInfo {
            value: "无畏战舰",
            positive: None,
        }),
        "guardian_sphere" => Some(LogicInfo {
            value: "无限神机",
            positive: None,
        }),
        "is_war_participant" => Some(LogicInfo {
            value: "是参战方",
            positive: None,
        }),
        "attackers" => Some(LogicInfo {
            value: "进攻方",
            positive: None,
        }),
        "any_war" => Some(LogicInfo {
            value: "任意战争",
            positive: None,
        }),
        "has_diplo_migration_treaty" => Some(LogicInfo {
            value: "有移民条约",
            positive: None,
        }),
        "has_non_aggression_pact" => Some(LogicInfo {
            value: "有互不侵犯条约",
            positive: None,
        }),
        "has_migration_control" => Some(LogicInfo {
            value: "有移民管控",
            positive: None,
        }),
        "has_citizenship_type" => Some(LogicInfo {
            value: "公民权类型为",
            positive: None,
        }),
        "has_citizenship_rights" => Some(LogicInfo {
            value: "有公民权",
            positive: None,
        }),
        "country" => Some(LogicInfo {
            value: "国家",
            positive: None,
        }),
        "nany_owned_planets" | "num_strategic_resources" => Some(LogicInfo {
            value: "任意拥有的行星",
            positive: None,
        }),
        "ruler" => Some(LogicInfo {
            value: "统治者",
            positive: None,
        }),
        "has_population_control" => Some(LogicInfo {
            value: "人口管制",
            positive: None,
        }),
        "num_species" => Some(LogicInfo {
            value: "种族数量",
            positive: None,
        }),
        "not_is_in_federation_with" => Some(LogicInfo {
            value: "与$has_a$不在同一联邦",
            positive: None,
        }),
        "not_is_exact_same_species" => Some(LogicInfo {
            value: "与$has_a$不严格相同的种族",
            positive: Some(false),
        }),
        "count_country" => Some(LogicInfo {
            value: "国家数量",
            positive: None,
        }),
        "add_modifier" => Some(LogicInfo {
            value: "添加修正",
            positive: None,
        }),
        "support" => Some(LogicInfo {
            value: "支持率",
            positive: None,
        }),
        "parameter:empire" => Some(LogicInfo {
            value: "国家参数",
            positive: None,
        }),
        "days" => Some(LogicInfo {
            value: "天数",
            positive: None,
        }),
        "every_pop_faction" => Some(LogicInfo {
            value: "每个派系",
            positive: None,
        }),
        "every_relation" => Some(LogicInfo {
            value: "每个已通信国家",
            positive: None,
        }),
        "leader_of_faction" => Some(LogicInfo {
            value: "派系领导人",
            positive: None,
        }),
        "count_pop_factions" => Some(LogicInfo {
            value: "派系人口数",
            positive: None,
        }),
        "is_country_type_with_subjects" => Some(LogicInfo {
            value: "国家有附庸国",
            positive: None,
        }),
        "every_system_within_border" => Some(LogicInfo {
            value: "疆域内所有星系",
            positive: None,
        }),
        "add_claims" => Some(LogicInfo {
            value: "增加s宣称",
            positive: None,
        }),
        "remove_claims" => Some(LogicInfo {
            value: "移出宣称",
            positive: None,
        }),
        "add_threat" => Some(LogicInfo {
            value: "增加威胁度",
            positive: None,
        }),
        "fallen_empire_humiliate_effect" => Some(LogicInfo {
            value: "失落帝国羞辱效果",
            positive: None,
        }),
        "is_valid_target_fe_stop_atrocities" => Some(LogicInfo {
            value: "目标是失落帝国施行§Y中止暴行§!的可用目标",
            positive: None,
        }),
        "is_valid_target_fe_stop_ai" => Some(LogicInfo {
            value: "目标是失落帝国施行§Y人工智能非法化§!的可用目标",
            positive: None,
        }),
        "is_valid_target_fe_cleanse_holy_worlds" => Some(LogicInfo {
            value: "目标是失落帝国施行§Y净化圣地§!的可用目标",
            positive: None,
        }),
        "every_playable_country" => Some(LogicInfo {
            value: "每个可玩国家",
            positive: None,
        }),
        "count_starbase_modules" => Some(LogicInfo {
            value: "恒星基地模块数量",
            positive: None,
        }),
        "has_starbase_building" => Some(LogicInfo {
            value: "有恒星基地建筑",
            positive: None,
        }),
        "country_naval_cap_add" => Some(LogicInfo {
            value: "海军容量上限增加",
            positive: None,
        }),
        "has_starbase_size" => Some(LogicInfo {
            value: "恒星基地等级",
            positive: None,
        }),
        "change_variable" => Some(LogicInfo {
            value: "修改变量",
            positive: None,
        }),
        "space_owner" => Some(LogicInfo {
            value: "恒星系所属者",
            positive: None,
        }),
        "has_event_chain" => Some(LogicInfo {
            value: "有事件链",
            positive: None,
        }),
        _ => None,
    }
}
