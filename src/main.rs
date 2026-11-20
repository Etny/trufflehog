// Welkom in Rust!!!!
// Ik heb veel geyapt, enjoy.
//
// Dit is voor later, negeer voor nu
mod hog;

// WAT DIT ALGORITME DOET
// We gaan op zoek naar de float met het laagste kwadraat!!!
// Het globale minimum is dus 0, maar dat weet ons programma
// niet, en we zullen dat best vaak ook niet als antwoord krijgen

// Dit is het "ding" waar we mee werken
// Wat het is maakt niet uit, in dit
// geval is het gewoon een float. Float
// heet in rust f32, omdat het 32 bits opneemt.
//
// Dit is een type alias, letterlijk gewoon
// een Find+Replace van 'Solution' naar 'f32'
type Solution = f32;

// Dit is de functie die we willen minimaliseren
// Naam | Args (naam: type) | Return type
// ||           ||           ||
// \/           \/           \/
fn cost(sol: &Solution) -> f32 {
    //       ^
    //       |
    // "My guy, betekent deze '&'????"
    //
    // Rust noemt dit een "reference", wat aangeeft
    // dat we een f32 willen "borrow"-en ipv helemaal
    // overkopieren. We verwachten als argument niet
    // een Solution zelf, maar een long (usize in Rust)
    // die als waarde het address heeft waar de data van
    // een Solution begint. Dit heeft 2 grote voordelen:
    //
    //      1.  We sturen minder data heen en weer. In
    //          dit geval is het letterlijk waardeloos,
    //          want een f32 is klein en zo gekopieerd,
    //          maar als Solution een daadwerkelijk data-
    //          structuur is dat veel ruimte opneemt, is
    //          het voor iedereen handiger om gewoon 8
    //          bytes door te sturen ipv het hele ding
    //          steeds te kopieren. Als je een vriend
    //          wilt bezoeken is het ook handiger om
    //          hun address te vragen, in plaats van
    //          dat je ze een kopie van hun huis voor
    //          je deur laat bouwen.
    //
    //      2.  Dit wordt later duidelijker, maar wij
    //          geven met deze syntax aan dat wij de
    //          waarde NIET gaan veranderen. Als je
    //          dat hier zou proberen, krijg je van
    //          Rust een error. Dit is top, omdat
    //          het duidelijk is wanneer je data
    //          aangepast wordt, en het is voor de
    //          compiler heel relevant voor optima-
    //          lisatie redenen

    // Uncomment me and you get an error!
    // *sol += 1.0;

    // Voor deze demo nemen we de waarde in het kwadraat.
    // Hiervoor gebruiken we de ingebouwde 'powi' functie,
    // die een float naar een integer-macht doet
    f32::powi(*sol, 2)
    //        ^
    //        |
    //  Bro wtf is dit sterretje?
    //  Simpel: sol is een &f32, niet een f32 zelf.
    //  Door '*' voor een & te zetten 'pak je hem uit'.
    //  Je gaat dus daadwerkelijk naar het geheugen
    //  waar de data staat, en leest het uit. (Dit
    //  heet dereferencing)
    //
    //  Het is een beetje verwarrend soms, omdat je
    //  de '*' in Rust heel vaak weg mag laten. De
    //  compiler geeft je hier wel hele lieve errors
    //  voor. Haal de '*' eens weg en run 'cargo build'
    //  om te zien wat de hoe handig de error is :)
}

// Dit is een datastructuur
//
// Dit is praktish hetzelfde als 'class' in C#,
// alleen worden struct definities opgesplitst:
// Hier zeggen we welke data we allemaal opslaan
// (dus wat in C# de member variabelen zijn), en
// in een 'impl' blok schrijven we alle functies
// die bij een struct horen
struct Solver {
    current: Solution, // Dit is onze huidige oplossing, vanuit waar we zoeken
    best: Solution,    // Dit is onze beste oplossing tot nu toe
    iters: u32,        // Dit is hoeveel iteraties we hebben gedaan
                       // 'u32' betekent 'unsigned 32-bit int', dus
                       // het kan alleen 0 of groter bevatten
}

// Lets go write some functions!
impl Solver {
    // Rust heeft GEEN constructors. Je maakt
    // gewoon zelf een 'static' functie die
    // een nieuwe kopie maakt van je datastructuur.
    // Meestal noemt men deze functie 'new'
    //
    // Alle functies in Rust zijn static. Je roept
    // ze dan aan met de syntax:
    //      TypeName::function()
    //  Deze functie roepen we dus aan als volgt:
    //      Solver::new()
    //
    // Merk op dat deze functie public is, omdat hier
    // links onder 'pub' staat
    pub fn new() -> Solver {
        // Dit is de Rust versie van 'new', je
        // vult de waarde van alle velden in.
        Solver {
            // We beginnen bij 1, als simpel voorbeeld
            current: 1.0,
            best: 1.0,
            iters: 0,
        }
    }

    // wtf is 'mut self' bro???
    pub fn solve(mut self) -> Solution {
        // Dit is een grote feature in Rust, en het is extreem belangrijk:
        //
        // ========================================================
        // JE MAG NIKS AANPASSEN TENZIJ HET MET 'mut' GEMARKEERT IS
        // ========================================================
        //
        // 'Mut' staat voor 'mutable' aka 'aanpasbaar'.
        // Ieder variabele en argument kunnen we 'mut' voor zetten.
        // Als we dat niet doen, kunnen we het niet aanpassen. EZ.
        //
        // "Waarom zou je niet alles als mut markeren?"
        // Er zijn een paar belangrijke restricties die voor nu
        // niet belangrijk zijn. Het is sowieso super handig om te weten
        // wanneer code je data wel/niet gaat aanpassen. Het heeft
        // ook PERFORMANCE IMPLICATIES (!!!!)
        //
        //
        // "Okay sure bro maar wtf is 'self'????"
        // Het is letterlijk gewoon hetzelfde als 'this' in C#.
        // In die taal krijgt iedere niet-static functie eigenlijk
        // stiekem een this-waarde mee als eerste variabele. Bijv
        // als je doet:
        //      " dit is een string ".trim();
        // doe je eigenlijk:
        //      String.trim(" dit is een string ");
        //
        // Rust verstopt dit niet. Als het eerste argument van
        // een functie in een 'impl' block self heet, dan is
        // dat wat voor de punt staat bij de aanroep, maar
        // je mag functies ook aanroepen alsof ze static zijn.

        // Ook hier weer 'mut', want we gaan dit aanpassen.
        // We mouden hier bij hoeveel cycles we geen verbetering
        // hebben gevonden
        //
        //  ||
        //  \/
        let mut looped_cycles = 0;

        // 'loop' is letterlijk een oneindige loop (tot je break-t)
        loop {
            // We doen een iteratie
            self.iter();

            // Dit had dus niet gekunt zonder 'mut'
            looped_cycles += 1;

            // De '&'-tjes hier betekenen dat we het address
            // van de data meegeven, niet de data zelf.
            //      ||                    ||
            //      \/                    \/
            if cost(&self.current) < cost(&self.best) {
                // We hebben een betere oplossing gevonden,
                // dus we gaan nog ff door
                looped_cycles = 0;

                // Dit had niet gekunt zonder 'self'
                // als 'mut' te zetten
                self.best = self.current;

                // Als we voor een hoop iteraties
                // geen verbetering vinden stoppen we
            } else if looped_cycles > 100000000 {
                println!("Bailing!");
                break;
            }
        }
        println!("Ran for {} iters", self.iters);
        // We returnen dus het besten wat we
        // gevonden hebben (don't forget: 'return'
        // is optioneel in Rust als je geen ';'
        // achter de laatste regel in een functie
        // zet)
        self.best
    }

    // "Wat is een &mut self?"
    //
    // Hier komt alles samen: we 'lenen' de Solver
    // instance waarop we gaan werken (die als het
    // 'self' argument wordt meegegeven), en omdat
    // we dingen in die Solver gaan aanpassen moeten
    // we het 'mut' markeren
    //
    // "Waarom &mut self en niet mut self?"
    // Dit leg ik op een later moment is. De korte
    // is dat de Solver instance na het aanroepen
    // van 'solve()' weggegooid zal worden, omdat
    // we geen reference nemen maar gewoon het hele
    // object. Dit is voor nu niet relevant.
    fn iter(&mut self) {
        self.iters += 1;

        // Met 'let' declareer je een variabele.
        // Het type is meestal optioneel, want
        // Rust kan het vaak zelf uitvogelen,
        // maar dat zou je er zo bij zetten:
        //      let next: Solution = ...
        let next = add_random(self.current);

        if cost(&next) < cost(&self.current) {
            self.current = next;
        }
    }
}

// Dit is de enige Neighborhood die we voor nu hebben:
// het telt letterlijk gewoon een random nummer tussen
// -0.2 en 0.2 bij de solution op.
fn add_random(sol: Solution) -> Solution {
    sol + (0.4 * fastrand::f32()) - 0.2
}

// Dit is de main functie, net zoals Main in
// C# is dit waar on programma begint
fn main() {
    // Print some art :)
    println!("{}", hog::HOG_ART);
    println!("Welcome to trufflehog!");

    // We maken een nieuw solver...
    let solver = Solver::new();
    // ...en pakken de oplossing
    let solution = solver.solve();

    // Dit is hoe je waardes print: println!
    // vervangt de n-de '{}' in de string door
    // het n-de argument dat je meegeeft na
    // de string
    println!("Found solution: {}", solution);
}
