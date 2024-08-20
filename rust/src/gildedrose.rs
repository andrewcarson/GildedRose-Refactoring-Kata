use std::cmp;
use std::fmt::{self, Display};

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in self.items.iter_mut() {
            if item.name == "Sulfuras, Hand of Ragnaros" {
                // Do nothing
            } else if item.name == "Aged Brie" {
                GildedRose::update_aged_brie_quality(item);
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                GildedRose::update_backstage_pass_quality(item);
            } else {
                GildedRose::update_normal_item_quality(item);
            }
        }
    }

    fn update_normal_item_quality(item: &mut Item) {
        item.sell_in -= 1;
        if item.sell_in >= 0 {
            item.quality = cmp::max(0, item.quality - 1);
        } else {
            item.quality = cmp::max(0, item.quality - 2);
        }
    }

    fn update_aged_brie_quality(brie: &mut Item) {
        if brie.quality < 50 {
            brie.quality += 1;
        }
        brie.sell_in -= 1;

        if brie.sell_in < 0 && brie.quality < 50 {
            // TODO: A double increment of the quality??
            brie.quality += 1;
        }
    }

    fn update_backstage_pass_quality(pass: &mut Item) {
        if pass.quality < 50 {
            pass.quality += 1;
            if pass.sell_in < 11 && pass.quality < 50 {
                pass.quality += 1;
            }

            if pass.sell_in < 6 && pass.quality < 50 {
                pass.quality += 1;
            }
        }
        pass.sell_in -= 1;
        if pass.sell_in < 0 {
            pass.quality = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("fixme", rose.items[0].name);
    }
}
