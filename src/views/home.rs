use crate::routes::USR;
use crate::Route;
use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12 space-y-10 text-gray-800 font-sans",
            h1 { class: "text-3xl md:text-4xl font-bold text-center leading-tight tracking-tight text-gray-900",
                "Human proteome distribution atlas for tissue-specific plasma proteome dynamics."
            }
            p { class: "text-center text-sm md:text-base text-gray-600 leading-relaxed",
                "Erik Malmström, Lars Malmström, Simon Hauri, Tirthankar Mohanty, Aaron Scott, Christofer Karlsson, Carlos Gueto-Tettay, Emma Åhrman, Shahab Nozohoor, Bobby Tingstedt, Sara Regner, Peter Elfving, Leif Bjermer, Andreas Forsvall, Alexander Doyle, Mattias Magnusson, Ingrid Hedenfalk, Päivi Kannisto, Christian Brandt, Emma Nilsson, Lars B. Dahlin, Johan Malm, Adam Linder, Emma Niméus, Johan Malmström "
            }
            h4 { class: "text-xl md:text-2xl font-semibold text-gray-800 border-b border-gray-300 pb-1",
                "Summary"
            }
            p { class: "text-base md:text-lg leading-relaxed text-gray-700",
                "The plasma proteome is maintained by the influx and efflux of proteins from surrounding organs and cells. To quantify the extent to which different organs and cells impact the plasma proteome in healthy and diseased conditions, we developed a mass-spectrometry-based proteomics strategy to infer the tissue origin of proteins detected in human plasma. We first constructed an extensive human proteome atlas from 18 vascularized organs and the 8 most abundant cell types in blood. The atlas was interfaced with previous RNA and protein atlases to objectively define proteome-wide protein-organ associations to infer the origin and enable the reproducible quantification of organ-specific proteins in plasma. We demonstrate that the resource can determine disease-specific quantitative changes of organ-enriched protein panels in six separate patient cohorts, including sepsis, pancreatitis, and myocardial injury. The strategy can be extended to other diseases to advance our understanding of the processes contributing to plasma proteome dynamics."
            }
            p { class: "text-sm text-gray-500 italic text-right",
                "Malmström et al., 2025, Cell 188, 1–13 May 15, 2025 DOI: "
                a {
                    class: "text-blue-800",
                    href: "https://doi.org/10.1016/j.cell.2025.03.013",
                    target: "_blank",
                    "https://doi.org/10.1016/j.cell.2025.03.013"
                }
            }
        }
    }
}
