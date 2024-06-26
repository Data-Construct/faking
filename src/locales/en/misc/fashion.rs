use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn terms() -> String {
	FASHION_TERMS[seeder::gen_range(0..FASHION_TERMS_LEN)].to_string()
}

static FASHION_TERMS: [&'static str; 244] = [
	"Accent",
	"Accessories",
	"Acid wash",
	"Academic costume",
	"Accent shades",
	"Accessory",
	"Accessorizing",
	"Active sportswear",
	"Achromatic colours",
	"Accouterments",
	"Adaptation",
	"Advanced colours",
	"Androgynous style",
	"Anime",
	"Anti Fashion ",
	"Antique style",
	"Anti Pill",
	"Army look",
	"Art deco",
	"Art Nouveau",
	"a la mode",
	"American Style",
	"Apparel",
	"Apparel manufacturing",
	"Asymmetrical",
	"Atelier",
	"Attire",
	"Athletic clothing",
	"Au naturel",
	"Avant-Garde",
	"Body clothes",
	"Balance / symmetry",
	"Bespoke",
	"Barfly apparel",
	"Baroque",
	"Beachwear",
	"Beaumonde",
	"Black tie event",
	"Bling",
	"Body-con clothing",
	"Boutique",
	"Boyfriend style",
	"Bib and tucker",
	"Border print",
	"Bohemian style of fashion (Boho)",
	"Business Formal dress code",
	"Business Casual dress code",
	"Bulky clothing",
	"Capsule wardrobe",
	"Cut of a garment",
	"Camouflage clothing",
	"Care label",
	"Casual wear",
	"Catwalk",
	"Channel suit",
	"Cheugy",
	"Chic",
	"Chinois",
	"Cine mode",
	"City wear",
	"Cocktail dress code",
	"Collection",
	"Colour coordination",
	"Classic",
	"Clique",
	"Collection",
	"Colour blocking",
	"Colour fast",
	"Colorway",
	"Core aesthetics",
	"Cosplay",
	"Country look",
	"Cool colours",
	"Contemporary style",
	"Continental style",
	"Contrast",
	"Conservative styles",
	"Cosmetic colours",
	"Cosmopolitan",
	"Costume",
	"Couturier",
	"Composite style",
	"Costume jewelry",
	"Cultural appropriation in fashion",
	"Dated fashion",
	"Décolletage",
	"Diffusion Line",
	"Distressing",
	"Draping",
	"Drop tail style",
	"Dandy style",
	"Deadstock",
	"Design elements",
	"Double Denim Trend",
	"Earth colors",
	"Eclectic style of fashion",
	"Electric colours",
	"Elegant dressing style",
	"Ensemble",
	"Eponymous fashion brand",
	"Ethnic",
	"Exotic",
	"Embellishing",
	"Ensemble",
	"Designer",
	"Display",
	"Effortless dressing",
	"Fabric swatches",
	"Fabric Finishes",
	"Fad",
	"Fashion 2.0",
	"Fashion capitals",
	"Fashion Cycle",
	"Fashion Faux Pas",
	"Fashion Forecast",
	"Fashion forward",
	"Fashion icon",
	"Fashion Line",
	"Fashion press",
	"Fashion show",
	"Fashion house",
	"Fashionista",
	"Fashion icon",
	"Fashion sense",
	"Fashion police",
	"Fashion sketch",
	"Fashion subcultures",
	"Fashion tribe",
	"Fabric Prints",
	"Face of the fabric & texture",
	"Fashion Label",
	"Fatigues",
	"Flat sketches",
	"Florals",
	"Formal clothing ( evening clothes)",
	"Foundation",
	"Garcon look",
	"Gender-queer",
	"Gradation",
	"Geek Chic",
	"Groufit",
	"Grunge",
	"Haberdashery",
	"High stepper",
	"Hand of a fabric",
	"Haute Couture ",
	"Heavy metal fashion",
	"High Fashion",
	"Hippie style",
	"Hosiery",
	"Hourglass figure",
	"House",
	"Hot number",
	"Imitation",
	"Iridescent colours",
	"Impact colours",
	"Jewel tones",
	"Kawaii",
	"Kitsch",
	"Knock Off",
	"Lagenlook style ( Layered style)",
	"Limited edition clothing",
	"Line",
	"Line sheet",
	"Look Book",
	"Long line clothing",
	"Loungewear",
	"Made to measure garments",
	"Marine style",
	"Mass produced / Fashion",
	"Masculine style",
	"Melange",
	"Minimalist",
	"Monotone clothing",
	"Mood Board",
	"Motif",
	"Muslin",
	"Neutral Colours",
	"Off the rack",
	"Ombre",
	"Over size",
	"Passe ",
	"Paninaro",
	"Panache",
	"Pantone colours",
	"Pattern",
	"Pret-a- porter",
	"Peek-a-boo style",
	"Peasant style",
	"Placement prints",
	"Post modern fashion",
	"Power dressing",
	"Preppy style",
	"Pre-spring collection",
	"Print on print",
	"Punk style",
	"Resort  wear (Cruise wear)",
	"Ready to wear or RTW",
	"Retro",
	"Retrofuturism",
	"Reversible clothing",
	"Rockability fashion",
	"Safari style",
	"Sample",
	"Silhouette",
	"Seamstress/ Sewer / Sewist",
	"Selvedge",
	"Sanforized clothing",
	"Sleek style",
	"Slip on clothes",
	"Slim fit",
	"Sloper",
	"Slogan",
	"Sportswear",
	"Statement Jewelry",
	"Street wear",
	"Stonewashed fabric",
	"Stylist (Fashion)",
	"Surfer look",
	"Sartorial",
	"Separates",
	"Supportive clothing",
	"Style",
	"Style Surfing",
	"Sweats",
	"Tailoring",
	"Throwback fashion",
	"Theme",
	"Theme board (mood board)",
	"Toile",
	"Tone on Tone",
	"Trend",
	"Trendy",
	"Trunk show",
	"Twin prints",
	"Unisex style",
	"Up cycled clothing",
	"Utilitarian clothing",
	"Vintage",
	"Wasp waist",
	"Warm colours",
	"Weft",
	"X-ray Fabric",
	"70s style",
];
static FASHION_TERMS_LEN: usize = FASHION_TERMS.len();
