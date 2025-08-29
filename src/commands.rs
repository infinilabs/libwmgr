use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    TopHalf,
    BottomHalf,
    LeftHalf,
    RightHalf,
    CenterHalf,

    TopLeftQuarter,
    TopRightQuarter,
    BottomLeftQuarter,
    BottomRightQuarter,

    TopLeftSixth,
    TopCenterSixth,
    TopRightSixth,
    BottomLeftSixth,
    BottomCenterSixth,
    BottomRightSixth,

    TopThird,
    MiddleThird,
    BottomThird,

    Center,

    FirstFourth,
    SecondFourth,
    ThirdFourth,
    LastFourth,

    FirstThird,
    CenterThird,
    LastThird,

    FirstTwoThirds,
    CenterTwoThirds,
    LastTwoThirds,

    FirstThreeFourths,
    CenterThreeFourths,
    LastThreeFourths,

    TopThreeFourths,
    BottomThreeFourths,

    TopTwoThirds,
    BottomTwoThirds,
    TopCenterTwoThirds,

    TopFirstFourth,
    TopSecondFourth,
    TopThirdFourth,
    TopLastFourth,

    MakeLarger,
    MakeSmaller,

    AlmostMaximize,
    Maximize,
    MaximizeWidth,
    MaximizeHeight,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    NextDesktop,
    PreviousDesktop,
    NextDisplay,
    PreviousDisplay,

    // Restore
    Restore,

    ToggleFullscreen,
}
