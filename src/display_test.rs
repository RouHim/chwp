use crate::display;

#[test]
fn get_total_resolution() {
    // GIVEN

    // WHEN requesting total desktop resolution
    let total_resolution = display::get_total_resolution();

    // THEN it should match
    assert_eq!(total_resolution, "3640x1920");
}

#[test]
fn get_display_resolutions() {
    // GIVEN

    // WHEN requesting total desktop resolution
    let total_resolution = display::get_display_resolutions();

    // THEN it should match
    assert_eq!(total_resolution, ["2560x1440", "1080x1920"]);
}
