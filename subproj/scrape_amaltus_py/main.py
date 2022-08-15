from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.wait import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import NoSuchElementException
from selenium.webdriver.remote.webelement import WebElement

from webdriver_auto_update import check_driver


import os
import os.path
import time
import logging
import datetime
import pathlib
import glob
import sys

G_CURRENT_DIR = pathlib.Path().resolve()
G_DOWNLOAD_DIR = f"{G_CURRENT_DIR}\download"


def get_webdriver() -> webdriver.Chrome:
    check_driver("")

    driver_path = f"{G_CURRENT_DIR}\chromedriver.exe"

    options = webdriver.ChromeOptions()
    dpref = {
        "download.default_directory": G_DOWNLOAD_DIR
    }
    options.add_experimental_option('prefs', dpref)
    # options.add_argument("headless")
    # options.add_argument("--enable-javascript")
    # options.add_argument("--disable-gpu")
    return webdriver.Chrome(driver_path, options=options)


G_DRIVER: webdriver.Chrome = get_webdriver()


def handle_sec_frame():
    wdw = WebDriverWait(G_DRIVER, 10)

    try:
        wdw.until(EC.presence_of_element_located(
            (By.XPATH, "//*[@id='main-message']/h1[contains(text(), 'Your connection is not private')]")))

        xpath_button_advanced = "//button[contains(@id, 'details-button')]"
        xpath_final_paragraph = "//*[@id='proceed-link']"
        button_advanced = wdw.until(EC.element_to_be_clickable(
            (By.XPATH, xpath_button_advanced)))
        button_advanced.click()

        final_paragraph = wdw.until(EC.element_to_be_clickable(
            (By.XPATH, xpath_final_paragraph)))
        final_paragraph.click()
    except NoSuchElementException as e:
        print(e)

    pass


# word entry
# - entry: str
# - subentry: [
#   - type: str
#   - meanings: [
#     - meaning: str
#   ]
# ]

class DictionaryEntry:
    class Etymology:  # Kind of like a subentry. There is always at least one
        word_class: str  # noun, verb, adjective

        pass
    entry: str
    etymologies: list[Etymology]

    pass


def to_dictionary_entry(nodes) -> DictionaryEntry:
    entry = {
        "entry": "ll",
        "subentry": [

        ],
    }
    pass


def handle_entry() -> DictionaryEntry:
    print("---\nhandle_entry:\n---\n\n")
    wdw = WebDriverWait(G_DRIVER, 1)
    xpath_base = "//*[@id='fragment-3']"
    xpath_list_elem = "//div[@id='fragment-3']/li"
    # //*[@id="fragment-3"]/li/text()[2]

    try:
        wdw.until(EC.presence_of_element_located((By.XPATH, xpath_base)))
    except Exception as e:
        print("EXCEPTION 0: ", e)

    list_elem = G_DRIVER.find_element(By.XPATH, xpath_list_elem)
    nodes = G_DRIVER.execute_script(
        "return arguments[0].childNodes", list_elem)
    text_nodes = []
    for node in nodes:
        if not isinstance(node, WebElement):
            _text = node['textContent'].strip()
            if _text:
                text_nodes.append(_text)
        else:
            node: WebElement
            text_nodes.append(f"<{node.tag_name}>{node.text}")
    print(text_nodes)

    return to_dictionary_entry(text_nodes)


def main():
    amaltus_general = "https://amaltus.com/"
    amaltus_kbd_to_rus = "https://amaltus.com/%d0%ba%d0%b0%d0%b1%d0%b0%d1%80%d0%b4%d0%b8%d0%bd%d0%be-%d1%80%d1%83%d1%81%d1%81%d0%ba%d0%b8%d0%b9-%d1%81%d0%bb%d0%be%d0%b2%d0%b0%d1%80%d1%8c/"
    amaltus_rus_to_kbd = "https://amaltus.com/%d1%80%d1%83%d1%81%d1%81%d0%ba%d0%be-%d0%ba%d0%b0%d0%b1%d0%b0%d1%80%d0%b4%d0%b8%d0%bd%d1%81%d0%ba%d0%b8%d0%b9-%d1%81%d0%bb%d0%be%d0%b2%d0%b0%d1%80%d1%8c-2/"
    G_DRIVER.get(amaltus_kbd_to_rus)
    handle_sec_frame()

    wdw = WebDriverWait(G_DRIVER, 1)
    xpath_button_sb = "//*[@id='sb_butt']"

    # Here we are in the main search page, and we click on the search button without any input
    # so that we get the whole list of words
    try:
        button_sb = wdw.until(EC.element_to_be_clickable(
            (By.XPATH, xpath_button_sb)))
        button_sb.click()

    except NoSuchElementException as e:
        print(e)

    ##############################################

    # Here we are now in the list of words.
    # We go through each of them
    xpath_base = "//*[@id='fragment-2']/li/ul/li[1]/a"
    xpath_elem = "//*[@id='fragment-2']/li/ul"
    try:
        wdw.until(EC.element_to_be_clickable(
            (By.XPATH, xpath_elem)))
    except NoSuchElementException as e:
        print(e)

    try:

        while True:
            elem_list = G_DRIVER.find_element(By.XPATH, xpath_elem)
            elems = elem_list.find_elements(By.TAG_NAME, "li")
            # print(len(elems)) # max 20
            for e in elems:
                entry_elem = e.find_element(By.TAG_NAME, "a")
                entry_elem.click()
                handle_entry()

                G_DRIVER.find_element(
                    By.XPATH, "//*[@id='tabs']/ul/li[2]/a").click()

            G_DRIVER.find_element(By.XPATH, "//*[@id='next_nav']").click()
            time.sleep(3)

    except NoSuchElementException as e:
        print(e)

    G_DRIVER.quit()


pass


if __name__ == "__main__":
    main()
