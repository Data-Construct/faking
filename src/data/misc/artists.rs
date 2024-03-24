use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn artist() -> String {
	ARTISTS[seeder::gen_range(0..ARTISTS_LEN)].to_string()
}

static ARTISTS: [&'static str; 42] = [
	"Donatello",
	"Botticelli",
	"Michelangelo",
	"Raphael",
	"Titian",
	"Durer",
	"Caravaggio",
	"Rubens",
	"Bernini",
	"Rembrandt",
	"Pissarro",
	"Manet",
	"Degas",
	"Cezanne",
	"Monet",
	"Renoir",
	"Cassatt",
	"Gauguin",
	"Munch",
	"Klimt",
	"Matisse",
	"Picasso",
	"Kandinsky",
	"Chagall",
	"Seurat",
	"Magritte",
	"Escher",
	"Rothko",
	"Dali",
	"Kahlo",
	"Pollock",
	"Warhol",
	"Vettriano",
	"Da Vinci",
	"El Greco",
	"Winslow Homer",
	"Paul Klee",
	"Edward Hopper",
	"Diego Rivera",
	"Vincent",
	"Joan Miro",
	"Ansel Adams",
];
static ARTISTS_LEN: usize = ARTISTS.len();
