pub const HELLO_WORLD: &str = "Hello world";
pub const BOOTSTRAP_NAVBAR: &str = "Bootstrap Navbar";
pub const BULMA_BOX: &str = "Bulma Box";
pub const HTML_TO_SEED: &str = "Html-to-Seed";

pub const HELLO_WORLD_HTML: &str = r##"<span>Hello world</span>"##;

pub const BOOTSTRAP_NAVBAR_HTML: &str = r##"<nav class="navbar navbar-expand-lg navbar-light bg-light">
    <a class="navbar-brand" href="#">Navbar</a>
    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent"
        aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
    </button>

    <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav mr-auto">
            <li class="nav-item active">
                <a class="nav-link" href="#">Home <span class="sr-only">(current)</span></a>
            </li>
            <li class="nav-item">
                <a class="nav-link" href="#">Link</a>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-toggle="dropdown"
                    aria-haspopup="true" aria-expanded="false">
                    Dropdown
                </a>
                <div class="dropdown-menu" aria-labelledby="navbarDropdown">
                    <a class="dropdown-item" href="#">Action</a>
                    <a class="dropdown-item" href="#">Another action</a>
                    <div class="dropdown-divider"></div>
                    <a class="dropdown-item" href="#">Something else here</a>
                </div>
            </li>
            <li class="nav-item">
                <a class="nav-link disabled" href="#">Disabled</a>
            </li>
        </ul>
        <form class="form-inline my-2 my-lg-0">
            <input class="form-control mr-sm-2" type="search" placeholder="Search" aria-label="Search">
            <button class="btn btn-outline-success my-2 my-sm-0" type="submit">Search</button>
        </form>
    </div>
</nav>
"##;

pub const BULMA_BOX_HTML: &str = r##"<div class="box">
    <article class="media">
        <div class="media-left">
            <figure class="image is-64x64">
                <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image">
            </figure>
        </div>
        <div class="media-content">
            <div class="content">
                <p>
                    <strong>John Smith</strong> <small>@johnsmith</small> <small>31m</small>
                    <br>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla
                    egestas. Nullam condimentum luctus turpis.
                </p>
            </div>
            <nav class="level is-mobile">
                <div class="level-left">
                    <a class="level-item">
                        <span class="icon is-small"><i class="fas fa-reply"></i></span>
                    </a>
                    <a class="level-item">
                        <span class="icon is-small"><i class="fas fa-retweet"></i></span>
                    </a>
                    <a class="level-item">
                        <span class="icon is-small"><i class="fas fa-heart"></i></span>
                    </a>
                </div>
            </nav>
        </div>
    </article>
</div>
"##;

pub const HTML_TO_SEED_HTML: &str = r##"<div class="bg-purple-600 flex flex-col h-screen text-gray-900">
    <div class="bg-purple-600 flex flex-grow-0 flex-row justify-between">
        <div class="flex flex-row">
            <h1 class="font-bold px-4 py-2 text-white text-2xl tracking-wider">Html to Seed</h1>
        </div>
        <a href="#"
            class="border border-white flex hover:bg-gray-800 hover:border-gray-800 items-center mr-3 my-3 px-3 rounded text-white text-xs">
            <img class="w-3 h-3 mr-1" src="../static/images/GitHub-Mark-Light-120px-plus.png" alt="Github logo">
            Github
        </a>
    </div>
    <div class="flex flex-grow-1 flex-row h-full mb-3">
        <div class="flex flex-col ml-3 w-1/2">
            <div class="bg-gray-200 py-2 rounded-t-lg text-center">
                <div class="flex-grow-0 relative group">
                    <div class="absolute cursor-pointer flex items-center left-0 pl-4 py-2 text-sm tracking-wider">
                        Samples
                        <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                            <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" />
                        </svg>
                    </div>
                    <div
                        class="absolute bg-white group-hover:visible invisible items-center mt-8 rounded-b-lg shadow-lg w-48">
                        <a href="#" class="px-4 py-2 block text-sm hover:text-white hover:bg-gray-800">Hello
                            world</a>
                        <a href="#" class="px-4 py-2 block text-sm hover:text-white hover:bg-gray-800">Bootstrap
                            Navbar</a>
                        <a href="#" class="px-4 py-2 block text-sm hover:text-white hover:bg-gray-800">Bulma
                            Box</a>
                    </div>
                </div>
                <div class="flex-grow-1 w-full">
                    <p class="py-2 text-sm">Type or paste HTML fragment</p>
                </div>
            </div>
            <textarea class="border font-mono h-full leading-tight text-sm" spellcheck="false"></textarea>
        </div>
        <div class="flex flex-col ml-3 mr-3 w-1/2">
            <div class="bg-gray-200 flex py-2 rounded-t-lg">
                <a class="absolute border border-gray-800 flex hover:bg-gray-900 hover:text-white ml-3 my-1 p-1 rounded text-sm"
                    href="#">
                    <svg class="fill-current h-5 ml-1 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
                        <path
                            d="M53.98 9.143h-3.97c-.082 0-.155.028-.232.047V5.023C49.778 2.253 47.473 0 44.64 0H10.217C7.384 0 5.08 2.253 5.08 5.023v46.843c0 2.77 2.305 5.023 5.138 5.023h6.037v2.268c0 2.67 2.216 4.843 4.941 4.843H53.98c2.725 0 4.942-2.173 4.942-4.843v-45.17c0-2.671-2.217-4.844-4.942-4.844zM7.11 51.866V5.023c0-1.649 1.394-2.991 3.106-2.991H44.64c1.712 0 3.106 1.342 3.106 2.99v46.844c0 1.649-1.394 2.991-3.106 2.991H10.217c-1.712 0-3.106-1.342-3.106-2.99zm49.778 7.29c0 1.551-1.306 2.812-2.91 2.812H21.195c-1.604 0-2.91-1.26-2.91-2.811v-2.268H44.64c2.833 0 5.138-2.253 5.138-5.023V11.128c.077.018.15.047.233.047h3.968c1.604 0 2.91 1.26 2.91 2.811v45.17z" />
                        <path
                            d="M38.603 13.206H16.254a1.015 1.015 0 1 0 0 2.032h22.35a1.015 1.015 0 1 0 0-2.032zM38.603 21.333H16.254a1.015 1.015 0 1 0 0 2.032h22.35a1.015 1.015 0 1 0 0-2.032zM38.603 29.46H16.254a1.015 1.015 0 1 0 0 2.032h22.35a1.015 1.015 0 1 0 0-2.032zM28.444 37.587h-12.19a1.015 1.015 0 1 0 0 2.032h12.19a1.015 1.015 0 1 0 0-2.032z" />
                    </svg>
                    <p class="ml-1 mr-1">Copy</p>
                </a>
                <p class="py-2 text-center text-sm w-full">Rust code compatible with Seed</p>
            </div>
            <textarea class="border font-mono h-full leading-tight text-sm" spellcheck="false"></textarea>
        </div>
    </div>
</div>
"##;
