
# Invention Mapper

Um mapa interativo mostrando em qual lugar do mundo invenções importantes foram criadas. Apesar do conceito besta a função deste projeto era estudar e praticar o uso de certas ferramentas, sendo elas: uma API construída em [Rust](https://www.rust-lang.org), a [API da Wikipedia](https://www.mediawiki.org/wiki/API:Main_page/pt-br) para coleta automática de dados, implementação de [Ollama](https://ollama.com) para resumir textos, um frontend construído usando [Astro](https://astro.build) e mapa feito através de [Leaflet](https://leafletjs.com).

As informações de como rodar a aplicação constam nas descrições das pastas server e web respectivamente.

![gif displaying landing application main page. The map is zooming in at paris (where Santos Dumont 14bis was first demonstrated) and an icon of an airplane is visible. When clicking at the icon a dialog appears from the bottom of the screen containing a description of the invention of airplanes, a link to further reading and a button to edit](.github/landing.gif)

Devido aos objetivos do estudo é possível incluir novas invenções tanto de maneira manual quanto apenas informando um link da Wikipedia.

![form to add a new invention. Form includes the following fields: name, image, description/information, reference link, latitude and longitude. Theres also a switch button to change between manual and wiki link forms](.github/form-manual.png)

![form to add a new invention. Form includes the following fields: wikipedia link, latitude and longitude. Theres also a switch button to change between manual and wiki link forms](.github/form-wiki.png)