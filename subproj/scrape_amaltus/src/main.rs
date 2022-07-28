use thirtyfour::{prelude::*, ChromeCapabilities};
use tokio;

async fn handle_sec_frame(driver: &WebDriver) {
    // Because amaltus is not secure we have to do some extra stuff.
    // Right now, the assumption is that it appears every time.
    // Maybe later, we can make this more robust.
    let private_text = driver
        .find(By::XPath(
            "//*[@id='main-message']/h1[contains(text(), 'Your connection is not private')]",
        ))
        .await;

    let _sec_window_exists = if private_text.is_ok() {
        println!("The security window is being handled");
        true
    } else {
        println!("The security window is not being handled");
        panic!();
        false
    };

    let advanced_button = driver
        .find(By::XPath("//button[contains(@id, 'details-button')]"))
        .await;
    //*[@id="details-button"]
    advanced_button.unwrap().click().await;

    let final_paragraph = driver.find(By::XPath("//*[@id='proceed-link']")).await;
    final_paragraph.unwrap().click().await;
}
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let amaltus_general = "https://amaltus.com/";
    let amaltus_kbd_to_rus = "https://amaltus.com/%d0%ba%d0%b0%d0%b1%d0%b0%d1%80%d0%b4%d0%b8%d0%bd%d0%be-%d1%80%d1%83%d1%81%d1%81%d0%ba%d0%b8%d0%b9-%d1%81%d0%bb%d0%be%d0%b2%d0%b0%d1%80%d1%8c/";
    let _amaltus_rus_to_kbd = "https://amaltus.com/%d1%80%d1%83%d1%81%d1%81%d0%ba%d0%be-%d0%ba%d0%b0%d0%b1%d0%b0%d1%80%d0%b4%d0%b8%d0%bd%d1%81%d0%ba%d0%b8%d0%b9-%d1%81%d0%bb%d0%be%d0%b2%d0%b0%d1%80%d1%8c-2/";
    driver.goto(amaltus_general).await?;

    handle_sec_frame(&driver).await;
    driver.goto(amaltus_kbd_to_rus).await?;

    {
        let sb_button = driver.find(By::XPath("//*[@id='sb_butt']")).await;
        let _ = sb_button.unwrap().click().await;

        let xpath_elem = "//*[@id='fragment-2']/li/ul/li[1]/a";
        let css_elem = "div[id='fragment-2'] > li > ul > li:nth-child(1) > a";
        let s = driver.find_all(By::Css(css_elem)).await;
        let s = s.unwrap();
        println!("s: {:#?}", s);
        driver.quit().await?;
    }

    // driver.goto("https://wikipedia.org").await?;
    // let elem_form = driver.find(By::Id("search-form")).await?;
    //
    // // Find element from element.
    // let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // // Type in the search terms.
    // elem_text.send_keys("selenium").await?;

    // // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;

    // // Look for header to implicitly wait for the page to load.
    // driver.find(By::ClassName("firstHeading")).await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    // // Always explicitly close the browser.
    // // driver.quit().await?;

    Ok(())
}
