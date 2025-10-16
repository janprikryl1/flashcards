use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="container mx-auto px-4 py-12">
            <div class="grid lg:grid-cols-2 gap-12 items-center mb-16">
                <div>
                    <h1 class="text-5xl mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        {"Učte se efektivně s flashcards"}
                    </h1>
                    <p class="text-xl text-gray-600 mb-8">
                        {"Vytvářejte kartičky, organizujte je do balíčků a učte se vlastním tempem. "}
                        {"Jednoduché a efektivní."}
                    </p>
                </div>

                <div class="relative h-[400px] rounded-2xl overflow-hidden shadow-2xl">
                    <img
                        class="w-full h-full object-cover"
                        src="https://images.unsplash.com/photo-1662304696102-efafa11b27c3?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w3Nzg4Nzd8MHwxfHNlYXJjaHwxfHxzdHVkeSUyMGxlYXJuaW5nJTIwYm9va3N8ZW58MXx8fHwxNzYwNTI3NTEzfDA&ixlib=rb-4.1.0&q=80&w=1080&utm_source=figma&utm_medium=referral"
                        alt="Studium s flashcards"
                    />
                </div>
            </div>
        </section>
    }
}
