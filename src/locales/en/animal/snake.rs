use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = animal_snake)]
pub fn snake() -> String {
	EN_SNAKE[seeder::gen_range(0..EN_SNAKE_LEN)].to_string()
}

static EN_SNAKE: [&'static str; 576] = [
    "Viper Adder",
    "Common adder",
    "Death Adder",
    "Desert death adder",
    "Horned adder",
    "Long-nosed adder",
    "Many-horned adder",
    "Mountain adder",
    "Mud adder",
    "Namaqua dwarf adder",
    "Nightingale adder",
    "Peringueys adder",
    "Puff adder",
    "African puff adder",
    "Rhombic night adder",
    "Sand adder",
    "Dwarf sand adder",
    "Namib dwarf sand adder",
    "Water adder",
    "Aesculapian snake",
    "Anaconda",
    "Bolivian anaconda",
    "De Schauensees anaconda",
    "Green anaconda",
    "Yellow anaconda",
    "Arafura file snake",
    "Asp",
    "European asp",
    "Egyptian asp",
    "African beaked snake",
    "Ball Python",
    "Bird snake",
    "Black-headed snake",
    "Mexican black kingsnake",
    "Black rat snake",
    "Black snake",
    "Red-bellied black snake",
    "Blind snake",
    "Brahminy blind snake",
    "Texas blind snake",
    "Western blind snake",
    "Boa",
    "Abaco Island boa",
    "Amazon tree boa",
    "Boa constrictor",
    "Cuban boa",
    "Dumeril boa",
    "Dwarf boa",
    "Emerald tree boa",
    "Hogg Island boa",
    "Jamaican boa",
    "Madagascar ground boa",
    "Madagascar tree boa",
    "Puerto Rican boa",
    "Rainbow boa",
    "Red-tailed boa",
    "Rosy boa",
    "Rubber boa",
    "Sand boa",
    "Tree boa",
    "Boiga",
    "Boomslang",
    "Brown snake",
    "Eastern brown snake",
    "Bull snake",
    "Bushmaster",
    "Dwarf beaked snake",
    "Rufous beaked snake",
    "Canebrake",
    "Cantil",
    "Cascabel",
    "Cat-eyed snake",
    "Banded cat-eyed snake",
    "Green cat-eyed snake",
    "Cat snake",
    "Andaman cat snake",
    "Beddomes cat snake",
    "Dog-toothed cat snake",
    "Forstens cat snake",
    "Gold-ringed cat snake",
    "Gray cat snake",
    "Many-spotted cat snake",
    "Tawny cat snake",
    "Chicken snake",
    "Coachwhip snake",
    "Cobra",
    "Andaman cobra",
    "Arabian cobra",
    "Asian cobra",
    "Banded water cobra",
    "Black-necked cobra",
    "Black-necked spitting cobra",
    "Black tree cobra",
    "Burrowing cobra",
    "Cape cobra",
    "Caspian cobra",
    "Congo water cobra",
    "Common cobra",
    "Eastern water cobra",
    "Egyptian cobra",
    "Equatorial spitting cobra",
    "False cobra",
    "False water cobra",
    "Forest cobra",
    "Gold tree cobra",
    "Indian cobra",
    "Indochinese spitting cobra",
    "Javan spitting cobra",
    "King cobra",
    "Mandalay cobra",
    "Mozambique spitting cobra",
    "North Philippine cobra",
    "Nubian spitting cobra",
    "Philippine cobra",
    "Red spitting cobra",
    "Rinkhals cobra",
    "Shield-nosed cobra",
    "Sinai desert cobra",
    "Southern Indonesian spitting cobra",
    "Southern Philippine cobra",
    "Southwestern black spitting cobra",
    "Snouted cobra",
    "Spectacled cobra",
    "Spitting cobra",
    "Storm water cobra",
    "Thai cobra",
    "Taiwan cobra",
    "Zebra spitting cobra",
    "Colletts snake",
    "Congo snake",
    "Copperhead",
    "American copperhead",
    "Australian copperhead",
    "Coral snake",
    "Arizona coral snake",
    "Beddomes coral snake",
    "Brazilian coral snake",
    "Cape coral snake",
    "Harlequin coral snake",
    "High Woods coral snake",
    "Malayan long-glanded coral snake",
    "Texas Coral Snake",
    "Western coral snake",
    "Corn snake",
    "South eastern corn snake",
    "Cottonmouth",
    "Crowned snake",
    "Cuban wood snake",
    "Eastern hognose snake",
    "Egg-eater",
    "Eastern coral snake",
    "Fer-de-lance",
    "Fierce snake",
    "Fishing snake",
    "Flying snake",
    "Golden tree snake",
    "Indian flying snake",
    "Moluccan flying snake",
    "Ornate flying snake",
    "Paradise flying snake",
    "Twin-Barred tree snake",
    "Banded Flying Snake",
    "Fox snake, three species of Pantherophis",
    "Forest flame snake",
    "Garter snake",
    "Checkered garter snake",
    "Common garter snake",
    "San Francisco garter snake",
    "Texas garter snake",
    "Cape gopher snake",
    "Grass snake",
    "Green snake",
    "Rough green snake",
    "Smooth green snake",
    "Ground snake",
    "Common ground snake",
    "Three-lined ground snake",
    "Western ground snake",
    "Habu",
    "Hognose snake",
    "Blonde hognose snake",
    "Dusty hognose snake",
    "Jans hognose snake",
    "Giant Malagasy hognose snake",
    "Mexican hognose snake",
    "South American hognose snake",
    "Hundred pacer",
    "Ikaheka snake",
    "Indigo snake",
    "Jamaican Tree Snake",
    "Keelback",
    "Asian keelback",
    "Assam keelback",
    "Black-striped keelback",
    "Buff striped keelback",
    "Burmese keelback",
    "Checkered keelback",
    "Common keelback",
    "Hill keelback",
    "Himalayan keelback",
    "Khasi Hills keelback",
    "Modest keelback",
    "Nicobar Island keelback",
    "Nilgiri keelback",
    "Orange-collared keelback",
    "Red-necked keelback",
    "Sikkim keelback",
    "Speckle-bellied keelback",
    "White-lipped keelback",
    "Wynaad keelback",
    "Yunnan keelback",
    "King brown",
    "King snake",
    "California kingsnake",
    "Desert kingsnake",
    "Grey-banded kingsnake",
    "North eastern king snake",
    "Prairie kingsnake",
    "Scarlet kingsnake",
    "Speckled kingsnake",
    "Krait",
    "Banded krait",
    "Blue krait",
    "Black krait",
    "Burmese krait",
    "Ceylon krait",
    "Indian krait",
    "Lesser black krait",
    "Malayan krait",
    "Many-banded krait",
    "Northeastern hill krait",
    "Red-headed krait",
    "Sind krait",
    "Large shield snake",
    "Lancehead",
    "Common lancehead",
    "Lora",
    "Grey Lora",
    "Lyre snake",
    "Baja California lyresnake",
    "Central American lyre snake",
    "Texas lyre snake",
    "Eastern lyre snake",
    "Machete savane",
    "Mamba",
    "Black mamba",
    "Green mamba",
    "Eastern green mamba",
    "Western green mamba",
    "Mamushi",
    "Mangrove snake",
    "Milk snake",
    "Moccasin snake",
    "Montpellier snake",
    "Mud snake",
    "Eastern mud snake",
    "Western mud snake",
    "Mussurana",
    "Night snake",
    "Cat-eyed night snake",
    "Texas night snake",
    "Nichell snake",
    "Narrowhead Garter Snake",
    "Nose-horned viper",
    "Rhinoceros viper",
    "Vipera ammodytes",
    "Parrot snake",
    "Mexican parrot snake",
    "Patchnose snake",
    "Perrotets shieldtail snake",
    "Pine snake",
    "Pipe snake",
    "Asian pipe snake",
    "Dwarf pipe snake",
    "Red-tailed pipe snake",
    "Python",
    "African rock python",
    "Amethystine python",
    "Angolan python",
    "Australian scrub python",
    "Ball python",
    "Bismarck ringed python",
    "Black headed python",
    "Blood python",
    "Boelen python",
    "Borneo short-tailed python",
    "Bredls python",
    "Brown water python",
    "Burmese python",
    "Calabar python",
    "Western carpet python",
    "Centralian carpet python",
    "Coastal carpet python",
    "Inland carpet python",
    "Jungle carpet python",
    "New Guinea carpet python",
    "Northwestern carpet python",
    "Southwestern carpet python",
    "Childrens python",
    "Dauan Island water python",
    "Desert woma python",
    "Diamond python",
    "Flinders python",
    "Green tree python",
    "Halmahera python",
    "Indian python",
    "Indonesian water python",
    "Macklots python",
    "Mollucan python",
    "Oenpelli python",
    "Olive python",
    "Papuan python",
    "Pygmy python",
    "Red blood python",
    "Reticulated python",
    "Kayaudi dwarf reticulated python",
    "Selayer reticulated python",
    "Rough-scaled python",
    "Royal python",
    "Savu python",
    "Spotted python",
    "Stimsons python",
    "Sumatran short-tailed python",
    "Tanimbar python",
    "Timor python",
    "Wetar Island python",
    "White-lipped python",
    "Brown white-lipped python",
    "Northern white-lipped python",
    "Southern white-lipped python",
    "Woma python",
    "Western woma python",
    "Queen snake",
    "Racer",
    "Bimini racer",
    "Buttermilk racer",
    "Eastern racer",
    "Eastern yellowbelly sad racer",
    "Mexican racer",
    "Southern black racer",
    "Tan racer",
    "West Indian racer",
    "Raddysnake",
    "Southwestern blackhead snake",
    "Rat snake",
    "Bairds rat snake",
    "Beauty rat snake",
    "Great Plains rat snake",
    "Green rat snake",
    "Japanese forest rat snake",
    "Japanese rat snake",
    "King rat snake",
    "Mandarin rat snake",
    "Persian rat snake",
    "Red-backed rat snake",
    "Twin-spotted rat snake",
    "Yellow-striped rat snake",
    "Manchurian Black Water Snake",
    "Rattlesnake",
    "Arizona black rattlesnake",
    "Aruba rattlesnake",
    "Chihuahuan ridge-nosed rattlesnake",
    "Coronado Island rattlesnake",
    "Durango rock rattlesnake",
    "Dusky pigmy rattlesnake",
    "Eastern diamondback rattlesnake",
    "Grand Canyon rattlesnake",
    "Great Basin rattlesnake",
    "Hopi rattlesnake",
    "Lance-headed rattlesnake",
    "Long-tailed rattlesnake",
    "Massasauga rattlesnake",
    "Mexican green rattlesnake",
    "Mexican west coast rattlesnake",
    "Midget faded rattlesnake",
    "Mojave rattlesnake",
    "Northern black-tailed rattlesnake",
    "Oaxacan small-headed rattlesnake",
    "Rattler",
    "Red diamond rattlesnake",
    "Southern Pacific rattlesnake",
    "Southwestern speckled rattlesnake",
    "Tancitaran dusky rattlesnake",
    "Tiger rattlesnake",
    "Timber rattlesnake",
    "Tropical rattlesnake",
    "Twin-spotted rattlesnake",
    "Uracoan rattlesnake",
    "Western diamondback rattlesnake",
    "Ribbon snake",
    "Rinkhals",
    "River jack",
    "Sea snake",
    "Annulated sea snake",
    "Beaked sea snake",
    "Duboiss sea snake",
    "Hardwickes sea snake",
    "Hook Nosed Sea Snake",
    "Olive sea snake",
    "Pelagic sea snake",
    "Stokes sea snake",
    "Yellow-banded sea snake",
    "Yellow-bellied sea snake",
    "Yellow-lipped sea snake",
    "Shield-tailed snake",
    "Sidewinder",
    "Colorado desert sidewinder",
    "Mojave desert sidewinder",
    "Sonoran sidewinder",
    "Small-eyed snake",
    "Smooth snake",
    "Brazilian smooth snake",
    "European smooth snake",
    "Stiletto snake",
    "Striped snake",
    "Japanese striped snake",
    "Sunbeam snake",
    "Taipan",
    "Central ranges taipan",
    "Coastal taipan",
    "Inland taipan",
    "Paupan taipan",
    "Tentacled snake",
    "Tic polonga",
    "Tiger snake",
    "Chappell Island tiger snake",
    "Common tiger snake",
    "Downs tiger snake",
    "Eastern tiger snake",
    "King Island tiger snake",
    "Kreffts tiger snake",
    "Peninsula tiger snake",
    "Tasmanian tiger snake",
    "Western tiger snake",
    "Tigre snake",
    "Tree snake",
    "Blandings tree snake",
    "Blunt-headed tree snake",
    "Brown tree snake",
    "Long-nosed tree snake",
    "Many-banded tree snake",
    "Northern tree snake",
    "Trinket snake",
    "Black-banded trinket snake",
    "Twig snake",
    "African twig snake",
    "Twin Headed King Snake",
    "Titanboa",
    "Urutu",
    "Vine snake",
    "Asian Vine Snake, Whip Snake",
    "American Vine Snake",
    "Mexican vine snake",
    "Viper",
    "Asp viper",
    "Bamboo viper",
    "Bluntnose viper",
    "Brazilian mud Viper",
    "Burrowing viper",
    "Bush viper",
    "Great Lakes bush viper",
    "Hairy bush viper",
    "Nitsches bush viper",
    "Rough-scaled bush viper",
    "Spiny bush viper",
    "Carpet viper",
    "Crossed viper",
    "Cyclades blunt-nosed viper",
    "Eyelash viper",
    "False horned viper",
    "Feas viper",
    "Fifty pacer",
    "Gaboon viper",
    "Hognosed viper",
    "Horned desert viper",
    "Horned viper",
    "Jumping viper",
    "Kaznakovs viper",
    "Leaf-nosed viper",
    "Leaf viper",
    "Levant viper",
    "Long-nosed viper",
    "McMahons viper",
    "Mole viper",
    "Palestine viper",
    "Pallas viper",
    "Palm viper",
    "Amazonian palm viper",
    "Black-speckled palm-pitviper",
    "Eyelash palm-pitviper",
    "Green palm viper",
    "Mexican palm-pitviper",
    "Guatemalan palm viper",
    "Honduran palm viper",
    "Siamese palm viper",
    "Side-striped palm-pitviper",
    "Yellow-lined palm viper",
    "Pit viper",
    "Banded pitviper",
    "Bamboo pitviper",
    "Barbours pit viper",
    "Black-tailed horned pit viper",
    "Bornean pitviper",
    "Brongersmas pitviper",
    "Brown spotted pitviper[4]",
    "Cantors pitviper",
    "Elegant pitviper",
    "Eyelash pit viper",
    "Fan-Si-Pan horned pitviper",
    "Flat-nosed pitviper",
    "Godmans pit viper",
    "Green tree pit viper",
    "Habu pit viper",
    "Hagens pitviper",
    "Horseshoe pitviper",
    "Jerdons pitviper",
    "Kanburian pit viper",
    "Kaulbacks lance-headed pitviper",
    "Kham Plateau pitviper",
    "Large-eyed pitviper",
    "Malabar rock pitviper",
    "Malayan pit viper",
    "Mangrove pit viper",
    "Mangshan pitviper",
    "Motuo bamboo pitviper",
    "Nicobar bamboo pitviper",
    "Philippine pitviper",
    "Pointed-scaled pit viper[5]",
    "Red-tailed bamboo pitviper",
    "Schultzes pitviper",
    "Stejnegers bamboo pitviper",
    "Sri Lankan pit viper",
    "Temple pit viper",
    "Tibetan bamboo pitviper",
    "Tiger pit viper",
    "Undulated pit viper",
    "Waglers pit viper",
    "Wirots pit viper",
    "Portuguese viper",
    "Saw-scaled viper",
    "Schlegels viper",
    "Sedge viper",
    "Sharp-nosed viper",
    "Snorkel viper",
    "Temple viper",
    "Tree viper",
    "Chinese tree viper",
    "Guatemalan tree viper",
    "Huttons tree viper",
    "Indian tree viper",
    "Large-scaled tree viper",
    "Malcolms tree viper",
    "Nitsches tree viper",
    "Popes tree viper",
    "Rough-scaled tree viper",
    "Rungwe tree viper",
    "Sumatran tree viper",
    "White-lipped tree viper",
    "Ursinis viper",
    "Western hog-nosed viper",
    "Wart snake",
    "Water moccasin",
    "Water snake",
    "Bocourts water snake",
    "Northern water snake",
    "Whip snake",
    "Long-nosed whip snake",
    "Wolf snake",
    "African wolf snake",
    "Barred wolf snake",
    "Worm snake",
    "Common worm snake",
    "Longnosed worm snake",
    "Wutu",
    "Yarara",
    "Zebra snake",
];
static EN_SNAKE_LEN: usize = EN_SNAKE.len();