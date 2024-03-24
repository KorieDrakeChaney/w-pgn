#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nag {
    NullAnnotation,
    GoodMove,
    PoorMove,
    VeryGoodMove,
    VeryPoorMove,
    SpeculativeMove,
    QuestionableMove,
    ForcedMove,
    SingularMove,
    WorstMove,
    DrawishPosition,
    EqualChancesQuietPosition,
    EqualChancesActivePosition,
    UnclearPosition,
    WhiteSlightAdvantage,
    BlackSlightAdvantage,
    WhiteModerateAdvantage,
    BlackModerateAdvantage,
    WhiteDecisiveAdvantage,
    BlackDecisiveAdvantage,
    WhiteCrushingAdvantage,
    BlackCrushingAdvantage,
    WhiteZugzwang,
    BlackZugzwang,
    WhiteSlightSpaceAdvantage,
    BlackSlightSpaceAdvantage,
    WhiteModerateSpaceAdvantage,
    BlackModerateSpaceAdvantage,
    WhiteDecisiveSpaceAdvantage,
    BlackDecisiveSpaceAdvantage,
    WhiteSlightTimeDevelopmentAdvantage,
    BlackSlightTimeDevelopmentAdvantage,
    WhiteModerateTimeDevelopmentAdvantage,
    BlackModerateTimeDevelopmentAdvantage,
    WhiteDecisiveTimeDevelopmentAdvantage,
    BlackDecisiveTimeDevelopmentAdvantage,
    WhiteInitiative,
    BlackInitiative,
    WhiteLastingInitiative,
    BlackLastingInitiative,
    WhiteAttack,
    BlackAttack,
    WhiteInsufficientCompensation,
    BlackInsufficientCompensation,
    WhiteSufficientCompensation,
    BlackSufficientCompensation,
    WhiteMoreThanAdequateCompensation,
    BlackMoreThanAdequateCompensation,
    WhiteSlightCentralControlAdvantage,
    BlackSlightCentralControlAdvantage,
    WhiteModerateCentralControlAdvantage,
    BlackModerateCentralControlAdvantage,
    WhiteDecisiveCentralControlAdvantage,
    BlackDecisiveCentralControlAdvantage,
    WhiteSlightKingsideControlAdvantage,
    BlackSlightKingsideControlAdvantage,
    WhiteModerateKingsideControlAdvantage,
    BlackModerateKingsideControlAdvantage,
    WhiteDecisiveKingsideControlAdvantage,
    BlackDecisiveKingsideControlAdvantage,
    WhiteSlightQueensideControlAdvantage,
    BlackSlightQueensideControlAdvantage,
    WhiteModerateQueensideControlAdvantage,
    BlackModerateQueensideControlAdvantage,
    WhiteDecisiveQueensideControlAdvantage,
    BlackDecisiveQueensideControlAdvantage,
    WhiteVulnerableFirstRank,
    BlackVulnerableFirstRank,
    WhiteWellProtectedFirstRank,
    BlackWellProtectedFirstRank,
    WhitePoorlyProtectedKing,
    BlackPoorlyProtectedKing,
    WhiteWellProtectedKing,
    BlackWellProtectedKing,
    WhitePoorlyPlacedKing,
    BlackPoorlyPlacedKing,
    WhiteWellPlacedKing,
    BlackWellPlacedKing,
    WhiteVeryWeakPawnStructure,
    BlackVeryWeakPawnStructure,
    WhiteModeratelyWeakPawnStructure,
    BlackModeratelyWeakPawnStructure,
    WhiteModeratelyStrongPawnStructure,
    BlackModeratelyStrongPawnStructure,
    WhiteVeryStrongPawnStructure,
    BlackVeryStrongPawnStructure,
    WhitePoorKnightPlacement,
    BlackPoorKnightPlacement,
    WhiteGoodKnightPlacement,
    BlackGoodKnightPlacement,
    WhitePoorBishopPlacement,
    BlackPoorBishopPlacement,
    WhiteGoodBishopPlacement,
    BlackGoodBishopPlacement,
    WhitePoorRookPlacement,
    BlackPoorRookPlacement,
    WhiteGoodRookPlacement,
    BlackGoodRookPlacement,
    WhitePoorQueenPlacement,
    BlackPoorQueenPlacement,
    WhiteGoodQueenPlacement,
    BlackGoodQueenPlacement,
    WhitePoorPieceCoordination,
    BlackPoorPieceCoordination,
    WhiteGoodPieceCoordination,
    BlackGoodPieceCoordination,
    WhitePlayedOpeningVeryPoorly,
    BlackPlayedOpeningVeryPoorly,
    WhitePlayedOpeningPoorly,
    BlackPlayedOpeningPoorly,
    WhitePlayedOpeningWell,
    BlackPlayedOpeningWell,
    WhitePlayedOpeningVeryWell,
    BlackPlayedOpeningVeryWell,
    WhitePlayedMiddleGameVeryPoorly,
    BlackPlayedMiddleGameVeryPoorly,
    WhitePlayedMiddleGamePoorly,
    BlackPlayedMiddleGamePoorly,
    WhitePlayedMiddleGameWell,
    BlackPlayedMiddleGameWell,
    WhitePlayedMiddleGameVeryWell,
    BlackPlayedMiddleGameVeryWell,
    WhitePlayedEndgameVeryPoorly,
    BlackPlayedEndgameVeryPoorly,
    WhitePlayedEndgamePoorly,
    BlackPlayedEndgamePoorly,
    WhitePlayedEndgameWell,
    BlackPlayedEndgameWell,
    WhitePlayedEndgameVeryWell,
    BlackPlayedEndgameVeryWell,
    WhiteSlightCounterplay,
    BlackSlightCounterplay,
    WhiteModerateCounterplay,
    BlackModerateCounterplay,
    WhiteDecisiveCounterplay,
    BlackDecisiveCounterplay,
    WhiteModerateTimeControlPressure,
    BlackModerateTimeControlPressure,
    WhiteSevereTimeControlPressure,
    BlackSevereTimeControlPressure,
}

impl std::fmt::Display for Nag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nag::NullAnnotation => write!(f, "Null Annotation"),
            Nag::GoodMove => write!(f, "Good Move"),
            Nag::PoorMove => write!(f, "Poor Move"),
            Nag::VeryGoodMove => write!(f, "Very Good Move"),
            Nag::VeryPoorMove => write!(f, "Very Poor Move"),
            Nag::SpeculativeMove => write!(f, "Speculative Move"),
            Nag::QuestionableMove => write!(f, "Questionable Move"),
            Nag::ForcedMove => write!(f, "Forced Move"),
            Nag::SingularMove => write!(f, "Singular Move"),
            Nag::WorstMove => write!(f, "Worst Move"),
            Nag::DrawishPosition => write!(f, "Drawish Position"),
            Nag::EqualChancesQuietPosition => write!(f, "Equal Chances, Quiet Position"),
            Nag::EqualChancesActivePosition => write!(f, "Equal Chances, Active Position"),
            Nag::UnclearPosition => write!(f, "Unclear Position"),
            Nag::WhiteSlightAdvantage => write!(f, "White has slight advantage"),
            Nag::BlackSlightAdvantage => write!(f, "Black has slight advantage"),
            Nag::WhiteModerateAdvantage => write!(f, "White has a moderate advantage"),
            Nag::BlackModerateAdvantage => write!(f, "Black has a moderate advantage"),
            Nag::WhiteDecisiveAdvantage => write!(f, "White has a decisive advantage"),
            Nag::BlackDecisiveAdvantage => write!(f, "Black has a decisive advantage"),
            Nag::WhiteCrushingAdvantage => write!(f, "White has a crushing advantage"),
            Nag::BlackCrushingAdvantage => write!(f, "Black has a crushing advantage"),
            Nag::WhiteZugzwang => write!(f, "White is in Zugzwang"),
            Nag::BlackZugzwang => write!(f, "Black is in Zugzwang"),
            Nag::WhiteSlightSpaceAdvantage => write!(f, "White has a slight space advantage"),
            Nag::BlackSlightSpaceAdvantage => write!(f, "Black has a slight space advantage"),
            Nag::WhiteModerateSpaceAdvantage => write!(f, "White has a moderate space advantage"),
            Nag::BlackModerateSpaceAdvantage => write!(f, "Black has a moderate space advantage"),
            Nag::WhiteDecisiveSpaceAdvantage => write!(f, "White has a decisive space advantage"),
            Nag::BlackDecisiveSpaceAdvantage => write!(f, "Black has a decisive space advantage"),
            Nag::WhiteSlightTimeDevelopmentAdvantage => {
                write!(f, "White has a slight time (development) advantage")
            }
            Nag::BlackSlightTimeDevelopmentAdvantage => {
                write!(f, "Black has a slight time (development) advantage")
            }
            Nag::WhiteModerateTimeDevelopmentAdvantage => {
                write!(f, "White has a moderate time (development) advantage")
            }
            Nag::BlackModerateTimeDevelopmentAdvantage => {
                write!(f, "Black has a moderate time (development) advantage")
            }
            Nag::WhiteDecisiveTimeDevelopmentAdvantage => {
                write!(f, "White has a decisive time (development) advantage")
            }
            Nag::BlackDecisiveTimeDevelopmentAdvantage => {
                write!(f, "Black has a decisive time (development) advantage")
            }
            Nag::WhiteInitiative => write!(f, "White has the initiative"),
            Nag::BlackInitiative => write!(f, "Black has the initiative"),
            Nag::WhiteLastingInitiative => write!(f, "White has a lasting initiative"),
            Nag::BlackLastingInitiative => write!(f, "Black has a lasting initiative"),
            Nag::WhiteAttack => write!(f, "White has the attack"),
            Nag::BlackAttack => write!(f, "Black has the attack"),
            Nag::WhiteInsufficientCompensation => write!(f, "White has insufficient compensation"),
            Nag::BlackInsufficientCompensation => write!(f, "Black has insufficient compensation"),
            Nag::WhiteSufficientCompensation => write!(f, "White has sufficient compensation"),
            Nag::BlackSufficientCompensation => write!(f, "Black has sufficient compensation"),
            Nag::WhiteMoreThanAdequateCompensation => {
                write!(f, "White has more than adequate compensation")
            }
            Nag::BlackMoreThanAdequateCompensation => {
                write!(f, "Black has more than adequate compensation")
            }
            Nag::WhiteSlightCentralControlAdvantage => {
                write!(f, "White has a slight central control advantage")
            }
            Nag::BlackSlightCentralControlAdvantage => {
                write!(f, "Black has a slight central control advantage")
            }
            Nag::WhiteModerateCentralControlAdvantage => {
                write!(f, "White has a moderate central control advantage")
            }
            Nag::BlackModerateCentralControlAdvantage => {
                write!(f, "Black has a moderate central control advantage")
            }
            Nag::WhiteDecisiveCentralControlAdvantage => {
                write!(f, "White has a decisive central control advantage")
            }
            Nag::BlackDecisiveCentralControlAdvantage => {
                write!(f, "Black has a decisive central control advantage")
            }
            Nag::WhiteSlightKingsideControlAdvantage => {
                write!(f, "White has a slight kingside control advantage")
            }
            Nag::BlackSlightKingsideControlAdvantage => {
                write!(f, "Black has a slight kingside control advantage")
            }
            Nag::WhiteModerateKingsideControlAdvantage => {
                write!(f, "White has a moderate kingside control advantage")
            }
            Nag::BlackModerateKingsideControlAdvantage => {
                write!(f, "Black has a moderate kingside control advantage")
            }
            Nag::WhiteDecisiveKingsideControlAdvantage => {
                write!(f, "White has a decisive kingside control advantage")
            }
            Nag::BlackDecisiveKingsideControlAdvantage => {
                write!(f, "Black has a decisive kingside control advantage")
            }
            Nag::WhiteSlightQueensideControlAdvantage => {
                write!(f, "White has a slight queenside control advantage")
            }
            Nag::BlackSlightQueensideControlAdvantage => {
                write!(f, "Black has a slight queenside control advantage")
            }
            Nag::WhiteModerateQueensideControlAdvantage => {
                write!(f, "White has a moderate queenside control advantage")
            }
            Nag::BlackModerateQueensideControlAdvantage => {
                write!(f, "Black has a moderate queenside control advantage")
            }
            Nag::WhiteDecisiveQueensideControlAdvantage => {
                write!(f, "White has a decisive queenside control advantage")
            }
            Nag::BlackDecisiveQueensideControlAdvantage => {
                write!(f, "Black has a decisive queenside control advantage")
            }
            Nag::WhiteVulnerableFirstRank => write!(f, "White has a vulnerable first rank"),
            Nag::BlackVulnerableFirstRank => write!(f, "Black has a vulnerable first rank"),
            Nag::WhiteWellProtectedFirstRank => write!(f, "White has a well-protected first rank"),
            Nag::BlackWellProtectedFirstRank => write!(f, "Black has a well-protected first rank"),
            Nag::WhitePoorlyProtectedKing => write!(f, "White has a poorly protected king"),
            Nag::BlackPoorlyProtectedKing => write!(f, "Black has a poorly protected king"),
            Nag::WhiteWellProtectedKing => write!(f, "White has a well-protected king"),
            Nag::BlackWellProtectedKing => write!(f, "Black has a well-protected king"),
            Nag::WhitePoorlyPlacedKing => write!(f, "White has a poorly placed king"),
            Nag::BlackPoorlyPlacedKing => write!(f, "Black has a poorly placed king"),
            Nag::WhiteWellPlacedKing => write!(f, "White has a well-placed king"),
            Nag::BlackWellPlacedKing => write!(f, "Black has a well-placed king"),
            Nag::WhiteVeryWeakPawnStructure => write!(f, "White has a very weak pawn structure"),
            Nag::BlackVeryWeakPawnStructure => write!(f, "Black has a very weak pawn structure"),
            Nag::WhiteModeratelyWeakPawnStructure => {
                write!(f, "White has a moderately weak pawn structure")
            }
            Nag::BlackModeratelyWeakPawnStructure => {
                write!(f, "Black has a moderately weak pawn structure")
            }
            Nag::WhiteModeratelyStrongPawnStructure => {
                write!(f, "White has a moderately strong pawn structure")
            }
            Nag::BlackModeratelyStrongPawnStructure => {
                write!(f, "Black has a moderately strong pawn structure")
            }
            Nag::WhiteVeryStrongPawnStructure => {
                write!(f, "White has a very strong pawn structure")
            }
            Nag::BlackVeryStrongPawnStructure => {
                write!(f, "Black has a very strong pawn structure")
            }
            Nag::WhitePoorKnightPlacement => write!(f, "White has poor knight placement"),
            Nag::BlackPoorKnightPlacement => write!(f, "Black has poor knight placement"),
            Nag::WhiteGoodKnightPlacement => write!(f, "White has good knight placement"),
            Nag::BlackGoodKnightPlacement => write!(f, "Black has good knight placement"),
            Nag::WhitePoorBishopPlacement => write!(f, "White has poor bishop placement"),
            Nag::BlackPoorBishopPlacement => write!(f, "Black has poor bishop placement"),
            Nag::WhiteGoodBishopPlacement => write!(f, "White has good bishop placement"),
            Nag::BlackGoodBishopPlacement => write!(f, "Black has good bishop placement"),
            Nag::WhitePoorRookPlacement => write!(f, "White has poor rook placement"),
            Nag::BlackPoorRookPlacement => write!(f, "Black has poor rook placement"),
            Nag::WhiteGoodRookPlacement => write!(f, "White has good rook placement"),
            Nag::BlackGoodRookPlacement => write!(f, "Black has good rook placement"),
            Nag::WhitePoorQueenPlacement => write!(f, "White has poor queen placement"),
            Nag::BlackPoorQueenPlacement => write!(f, "Black has poor queen placement"),
            Nag::WhiteGoodQueenPlacement => write!(f, "White has good queen placement"),
            Nag::BlackGoodQueenPlacement => write!(f, "Black has good queen placement"),
            Nag::WhitePoorPieceCoordination => write!(f, "White has poor piece coordination"),
            Nag::BlackPoorPieceCoordination => write!(f, "Black has poor piece coordination"),
            Nag::WhiteGoodPieceCoordination => write!(f, "White has good piece coordination"),
            Nag::BlackGoodPieceCoordination => write!(f, "Black has good piece coordination"),
            Nag::WhitePlayedOpeningVeryPoorly => {
                write!(f, "White has played the opening very poorly")
            }
            Nag::BlackPlayedOpeningVeryPoorly => {
                write!(f, "Black has played the opening very poorly")
            }
            Nag::WhitePlayedOpeningPoorly => write!(f, "White has played the opening poorly"),
            Nag::BlackPlayedOpeningPoorly => write!(f, "Black has played the opening poorly"),
            Nag::WhitePlayedOpeningWell => write!(f, "White has played the opening well"),
            Nag::BlackPlayedOpeningWell => write!(f, "Black has played the opening well"),
            Nag::WhitePlayedOpeningVeryWell => write!(f, "White has played the opening very well"),
            Nag::BlackPlayedOpeningVeryWell => write!(f, "Black has played the opening very well"),
            Nag::WhitePlayedMiddleGameVeryPoorly => {
                write!(f, "White has played the middle game very poorly")
            }
            Nag::BlackPlayedMiddleGameVeryPoorly => {
                write!(f, "Black has played the middle game very poorly")
            }
            Nag::WhitePlayedMiddleGamePoorly => {
                write!(f, "White has played the middle game poorly")
            }
            Nag::BlackPlayedMiddleGamePoorly => {
                write!(f, "Black has played the middle game poorly")
            }
            Nag::WhitePlayedMiddleGameWell => write!(f, "White has played the middle game well"),
            Nag::BlackPlayedMiddleGameWell => write!(f, "Black has played the middle game well"),
            Nag::WhitePlayedMiddleGameVeryWell => {
                write!(f, "White has played the middle game very well")
            }
            Nag::BlackPlayedMiddleGameVeryWell => {
                write!(f, "Black has played the middle game very well")
            }
            Nag::WhitePlayedEndgameVeryPoorly => {
                write!(f, "White has played the endgame very poorly")
            }
            Nag::BlackPlayedEndgameVeryPoorly => {
                write!(f, "Black has played the endgame very poorly")
            }
            Nag::WhitePlayedEndgamePoorly => write!(f, "White has played the endgame poorly"),
            Nag::BlackPlayedEndgamePoorly => write!(f, "Black has played the endgame poorly"),
            Nag::WhitePlayedEndgameWell => write!(f, "White has played the endgame well"),
            Nag::BlackPlayedEndgameWell => write!(f, "Black has played the endgame well"),
            Nag::WhitePlayedEndgameVeryWell => write!(f, "White has played the endgame very well"),
            Nag::BlackPlayedEndgameVeryWell => write!(f, "Black has played the endgame very well"),
            Nag::WhiteSlightCounterplay => write!(f, "White has slight counterplay"),
            Nag::BlackSlightCounterplay => write!(f, "Black has slight counterplay"),
            Nag::WhiteModerateCounterplay => write!(f, "White has moderate counterplay"),
            Nag::BlackModerateCounterplay => write!(f, "Black has moderate counterplay"),
            Nag::WhiteDecisiveCounterplay => write!(f, "White has decisive counterplay"),
            Nag::BlackDecisiveCounterplay => write!(f, "Black has decisive counterplay"),
            Nag::WhiteModerateTimeControlPressure => {
                write!(f, "White has moderate time control pressure")
            }
            Nag::BlackModerateTimeControlPressure => {
                write!(f, "Black has moderate time control pressure")
            }
            Nag::WhiteSevereTimeControlPressure => {
                write!(f, "White has severe time control pressure")
            }
            Nag::BlackSevereTimeControlPressure => {
                write!(f, "Black has severe time control pressure")
            }
        }
    }
}

impl From<&str> for Nag {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        if let Some(dollar_sign) = chars.next() {
            if dollar_sign != '$' {
                return Nag::NullAnnotation;
            }
        } else {
            return Nag::NullAnnotation;
        }

        let number: String = chars.collect();

        Nag::from(number.parse::<u8>().unwrap_or(0))
    }
}

impl From<u8> for Nag {
    fn from(value: u8) -> Self {
        match value {
            1 => Nag::GoodMove,
            2 => Nag::PoorMove,
            3 => Nag::VeryGoodMove,
            4 => Nag::VeryPoorMove,
            5 => Nag::SpeculativeMove,
            6 => Nag::QuestionableMove,
            7 => Nag::ForcedMove,
            8 => Nag::SingularMove,
            9 => Nag::WorstMove,
            10 => Nag::DrawishPosition,
            11 => Nag::EqualChancesQuietPosition,
            12 => Nag::EqualChancesActivePosition,
            13 => Nag::UnclearPosition,
            14 => Nag::WhiteSlightAdvantage,
            15 => Nag::BlackSlightAdvantage,
            16 => Nag::WhiteModerateAdvantage,
            17 => Nag::BlackModerateAdvantage,
            18 => Nag::WhiteDecisiveAdvantage,
            19 => Nag::BlackDecisiveAdvantage,
            20 => Nag::WhiteCrushingAdvantage,
            21 => Nag::BlackCrushingAdvantage,
            22 => Nag::WhiteZugzwang,
            23 => Nag::BlackZugzwang,
            24 => Nag::WhiteSlightSpaceAdvantage,
            25 => Nag::BlackSlightSpaceAdvantage,
            26 => Nag::WhiteModerateSpaceAdvantage,
            27 => Nag::BlackModerateSpaceAdvantage,
            28 => Nag::WhiteDecisiveSpaceAdvantage,
            29 => Nag::BlackDecisiveSpaceAdvantage,
            30 => Nag::WhiteSlightTimeDevelopmentAdvantage,
            31 => Nag::BlackSlightTimeDevelopmentAdvantage,
            32 => Nag::WhiteModerateTimeDevelopmentAdvantage,
            33 => Nag::BlackModerateTimeDevelopmentAdvantage,
            34 => Nag::WhiteDecisiveTimeDevelopmentAdvantage,
            35 => Nag::BlackDecisiveTimeDevelopmentAdvantage,
            36 => Nag::WhiteInitiative,
            37 => Nag::BlackInitiative,
            38 => Nag::WhiteLastingInitiative,
            39 => Nag::BlackLastingInitiative,
            40 => Nag::WhiteAttack,
            41 => Nag::BlackAttack,
            42 => Nag::WhiteInsufficientCompensation,
            43 => Nag::BlackInsufficientCompensation,
            44 => Nag::WhiteSufficientCompensation,
            45 => Nag::BlackSufficientCompensation,
            46 => Nag::WhiteMoreThanAdequateCompensation,
            47 => Nag::BlackMoreThanAdequateCompensation,
            48 => Nag::WhiteSlightCentralControlAdvantage,
            49 => Nag::BlackSlightCentralControlAdvantage,
            50 => Nag::WhiteModerateCentralControlAdvantage,
            51 => Nag::BlackModerateCentralControlAdvantage,
            52 => Nag::WhiteDecisiveCentralControlAdvantage,
            53 => Nag::BlackDecisiveCentralControlAdvantage,
            54 => Nag::WhiteSlightKingsideControlAdvantage,
            55 => Nag::BlackSlightKingsideControlAdvantage,
            56 => Nag::WhiteModerateKingsideControlAdvantage,
            57 => Nag::BlackModerateKingsideControlAdvantage,
            58 => Nag::WhiteDecisiveKingsideControlAdvantage,
            59 => Nag::BlackDecisiveKingsideControlAdvantage,
            60 => Nag::WhiteSlightQueensideControlAdvantage,
            61 => Nag::BlackSlightQueensideControlAdvantage,
            62 => Nag::WhiteModerateQueensideControlAdvantage,
            63 => Nag::BlackModerateQueensideControlAdvantage,
            64 => Nag::WhiteDecisiveQueensideControlAdvantage,
            65 => Nag::BlackDecisiveQueensideControlAdvantage,
            66 => Nag::WhiteVulnerableFirstRank,
            67 => Nag::BlackVulnerableFirstRank,
            68 => Nag::WhiteWellProtectedFirstRank,
            69 => Nag::BlackWellProtectedFirstRank,
            70 => Nag::WhitePoorlyProtectedKing,
            71 => Nag::BlackPoorlyProtectedKing,
            72 => Nag::WhiteWellProtectedKing,
            73 => Nag::BlackWellProtectedKing,
            74 => Nag::WhitePoorlyPlacedKing,
            75 => Nag::BlackPoorlyPlacedKing,
            76 => Nag::WhiteWellPlacedKing,
            77 => Nag::BlackWellPlacedKing,
            78 => Nag::WhiteVeryWeakPawnStructure,
            79 => Nag::BlackVeryWeakPawnStructure,
            80 => Nag::WhiteModeratelyWeakPawnStructure,
            81 => Nag::BlackModeratelyWeakPawnStructure,
            82 => Nag::WhiteModeratelyStrongPawnStructure,
            83 => Nag::BlackModeratelyStrongPawnStructure,
            84 => Nag::WhiteVeryStrongPawnStructure,
            85 => Nag::BlackVeryStrongPawnStructure,
            86 => Nag::WhitePoorKnightPlacement,
            87 => Nag::BlackPoorKnightPlacement,
            88 => Nag::WhiteGoodKnightPlacement,
            89 => Nag::BlackGoodKnightPlacement,
            90 => Nag::WhitePoorBishopPlacement,
            91 => Nag::BlackPoorBishopPlacement,
            92 => Nag::WhiteGoodBishopPlacement,
            93 => Nag::BlackGoodBishopPlacement,
            94 => Nag::WhitePoorRookPlacement,
            95 => Nag::BlackPoorRookPlacement,
            96 => Nag::WhiteGoodRookPlacement,
            97 => Nag::BlackGoodRookPlacement,
            98 => Nag::WhitePoorQueenPlacement,
            99 => Nag::BlackPoorQueenPlacement,
            100 => Nag::WhiteGoodQueenPlacement,
            101 => Nag::BlackGoodQueenPlacement,
            102 => Nag::WhitePoorPieceCoordination,
            103 => Nag::BlackPoorPieceCoordination,
            104 => Nag::WhiteGoodPieceCoordination,
            105 => Nag::BlackGoodPieceCoordination,
            106 => Nag::WhitePlayedOpeningVeryPoorly,
            107 => Nag::BlackPlayedOpeningVeryPoorly,
            108 => Nag::WhitePlayedOpeningPoorly,
            109 => Nag::BlackPlayedOpeningPoorly,
            110 => Nag::WhitePlayedOpeningWell,
            111 => Nag::BlackPlayedOpeningWell,
            112 => Nag::WhitePlayedOpeningVeryWell,
            113 => Nag::BlackPlayedOpeningVeryWell,
            114 => Nag::WhitePlayedMiddleGameVeryPoorly,
            115 => Nag::BlackPlayedMiddleGameVeryPoorly,
            116 => Nag::WhitePlayedMiddleGamePoorly,
            117 => Nag::BlackPlayedMiddleGamePoorly,
            118 => Nag::WhitePlayedMiddleGameWell,
            119 => Nag::BlackPlayedMiddleGameWell,
            120 => Nag::WhitePlayedMiddleGameVeryWell,
            121 => Nag::BlackPlayedMiddleGameVeryWell,
            122 => Nag::WhitePlayedEndgameVeryPoorly,
            123 => Nag::BlackPlayedEndgameVeryPoorly,
            124 => Nag::WhitePlayedEndgamePoorly,
            125 => Nag::BlackPlayedEndgamePoorly,
            126 => Nag::WhitePlayedEndgameWell,
            127 => Nag::BlackPlayedEndgameWell,
            128 => Nag::WhitePlayedEndgameVeryWell,
            129 => Nag::BlackPlayedEndgameVeryWell,
            130 => Nag::WhiteSlightCounterplay,
            131 => Nag::BlackSlightCounterplay,
            132 => Nag::WhiteModerateCounterplay,
            133 => Nag::BlackModerateCounterplay,
            134 => Nag::WhiteDecisiveCounterplay,
            135 => Nag::BlackDecisiveCounterplay,
            136 => Nag::WhiteModerateTimeControlPressure,
            137 => Nag::BlackModerateTimeControlPressure,
            138 => Nag::WhiteSevereTimeControlPressure,
            139 => Nag::BlackSevereTimeControlPressure,
            _ => Nag::NullAnnotation,
        }
    }
}
