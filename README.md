# eframe template

Bu, [egui](https://github.com/emilk/egui/) kullanarak uygulamalar yazmak için bir framework olan [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) için hazırlanmış bir şablon deposudur (template repo).

Hedef, bunun Rust ile bir GUI uygulaması yazmaya başlamanın en basit yolu olmasıdır.

Uygulamanızı yerel (native) olarak veya web için derleyebilir ve GitHub Pages kullanarak paylaşabilirsiniz.

## Başlangıç

https://github.com/emilk/eframe_template/ adresindeki "Use this template" butonuna tıklayarak veya [bu talimatları](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template) izleyerek başlayın.

Crate adını değiştirin: Projeniz için iyi bir isim seçin ve aşağıdaki dosyalarda ismi değiştirin:

* `Cargo.toml`
* `package.name` kısmını `eframe_template` yerine `sizin_crate_adiniz` yapın.
* `package.authors` kısmını değiştirin.


* `main.rs`
* `eframe_template::TemplateApp` kısmını `sizin_crate_adiniz::TemplateApp` olarak değiştirin.


* `index.html`
* `<title>eframe template</title>` kısmını `<title>sizin_crate_adiniz</title>` olarak değiştirin (isteğe bağlı).


* `assets/sw.js`
* `'./eframe_template.js'` kısmını `./sizin_crate_adiniz.js` olarak değiştirin (`filesToCache` dizisi içinde).
* `'./eframe_template_bg.wasm'` kısmını `./sizin_crate_adiniz_bg.wasm` olarak değiştirin (`filesToCache` dizisi içinde).



Alternatif olarak, gerekli isimleri ve e-postayı soracak ve yukarıdaki yamaları sizin için yapacak olan `fill_template.sh` dosyasını çalıştırabilirsiniz. Bu, özellikle bu depoyu GitHub dışında klonlarsanız ve bu nedenle GitHub'ın şablon oluşturma özelliğini kullanamıyorsanız yararlıdır.

### egui Hakkında Öğrenilecekler

`src/app.rs` basit bir örnek uygulama içerir. Bu sadece ilham vermek içindir - isterseniz çoğunu kaldırabilirsiniz.

Resmi egui belgeleri [https://docs.rs/egui](https://docs.rs/egui) adresindedir. Video tanıtımını tercih ederseniz, [https://www.youtube.com/watch?v=NtUkr_z7l84](https://www.youtube.com/watch?v=NtUkr_z7l84) adresine göz atın. İlham almak için [egui web demosuna](https://emilk.github.io/egui/index.html) bakın ve içindeki kaynak kod bağlantılarını takip edin.

### Yerelde Test Etme

`cargo run --release`

Linux üzerinde öncelikle şunu çalıştırmanız gerekir:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

Fedora Rawhide üzerinde şunu çalıştırmanız gerekir:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Yerelde Web

Uygulamanızı [WASM](https://en.wikipedia.org/wiki/WebAssembly) olarak derleyebilir ve bir web sayfası olarak yayınlayabilirsiniz.

Web hedefi için derleme yapmak amacıyla [Trunk](https://trunkrs.dev/) kullanıyoruz.

1. `rustup target add wasm32-unknown-unknown` ile gerekli hedefi yükleyin.
2. `cargo install --locked trunk` ile Trunk'ı yükleyin.
3. Derlemek ve `http://127.0.0.1:8080` adresinde sunmak için `trunk serve` komutunu çalıştırın. Trunk, projeyi düzenlediğinizde otomatik olarak yeniden derleyecektir.
4. Bir tarayıcıda `http://127.0.0.1:8080/index.html#dev` adresini açın. Aşağıdaki uyarıya bakın.

> `assets/sw.js` betiği uygulamamızı önbelleğe almaya çalışacak ve sunucuya bağlanamadığında önbelleğe alınmış sürümü yükleyerek uygulamanızın çevrimdışı çalışmasına (PWA gibi) olanak tanıyacaktır.
> `index.html` dosyasına `#dev` eklemek bu önbelleğe almayı atlayacak ve geliştirme sırasında en son derlemeleri yüklememize izin verecektir.

### Web Dağıtımı (Deploy)

1. Sadece `trunk build --release` komutunu çalıştırın.
2. Bu, "static html" web sitesi olarak bir `dist` dizini oluşturacaktır.
3. `dist` dizinini, [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site) dahil olmak üzere çok sayıda ücretsiz barındırma web sitesinden herhangi birine yükleyin.
4. Etkinleştirirseniz uygulamamızı otomatik olarak GitHub Pages'e dağıtan bir iş akışını (workflow) zaten sağlıyoruz.

> Github Pages'i etkinleştirmek için Repository -> Settings -> Pages -> Source kısmına gidip `gh-pages` dalı ve `/` (root) olarak ayarlamanız gerekir.
> Eğer `gh-pages` dalı `Source` kısmında mevcut değilse, sadece `gh-pages` adında bir dal oluşturup push edin, ardından kullanılabilir olacaktır.
> Eğer `main` dalının adını başka bir şeyle değiştirdiyseniz (örneğin depoyu başlangıç dalı `master` olacak şekilde yeniden başlattıysanız), değişikliği yansıtmak için `.github/workflows/pages.yml` dosyasını düzenlediğinizden emin olun:
> ```yml
> on:
>   push:
>     branches:
>       - <dal adı>
> 
> ```
> 
> 

Şablon uygulamayı [https://emilk.github.io/eframe_template/](https://emilk.github.io/eframe_template/) adresinde test edebilirsiniz.

## egui Güncelleme

2023 itibarıyla egui, bozucu değişiklikler içeren sık sürümlerle aktif geliştirme aşamasındadır. [eframe_template](https://github.com/emilk/eframe_template/), her zaman egui'nin en son sürümünü kullanacak şekilde eş zamanlı olarak güncellenecektir.

`egui` ve `eframe` güncellerken, bunu her seferinde bir sürüm atlayarak yapmanız ve değişiklikler hakkında [egui sürüm notlarını](https://github.com/emilk/egui/blob/master/CHANGELOG.md) ve [eframe sürüm notlarını](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md) okumanız önerilir.
