## Appendix G - How Rust is Made and “Nightly Rust”

### Stability WITHOUT Stagnation

* == guiding principle: "you should NEVER have fear to upgrading -- to a -- NEW Rust stable version
* / EACH upgrade
  * painless
  * bring
    * NEW features
    * fewer bugs
    * faster compile times

### Choo, Choo! Release Channels and Riding the Trains

TODO: 
Rust development operates on a *train schedule*
* That is, all development is
done on the `master` branch of the Rust repository
* Releases follow a software
release train model, which has been used by Cisco IOS and other software
projects
* There are three *release channels* for Rust:

* Nightly
* Beta
* Stable

Most Rust developers primarily use the stable channel, but those who want to
try out experimental new features may use nightly or beta.

Here’s an example of how the development and release process works: let’s
assume that the Rust team is working on the release of Rust 1.5
* That release
happened in December of 2015, but it will provide us with realistic version
numbers
* A new feature is added to Rust: a new commit lands on the `master`
branch
* Each night, a new nightly version of Rust is produced
* Every day is a
release day, and these releases are created by our release infrastructure
automatically
* So as time passes, our releases look like this, once a night:

```text
nightly: * - - * - - *
```

Every six weeks, it’s time to prepare a new release! The `beta` branch of the
Rust repository branches off from the `master` branch used by nightly
* Now,
there are two releases:

```text
nightly: * - - * - - *
                     |
beta:                *
```

Most Rust users do not use beta releases actively, but test against beta in
their CI system to help Rust discover possible regressions
* In the meantime,
there’s still a nightly release every night:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Let’s say a regression is found
* Good thing we had some time to test the beta
release before the regression snuck into a stable release! The fix is applied
to `master`, so that nightly is fixed, and then the fix is backported to the
`beta` branch, and a new release of beta is produced:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Six weeks after the first beta was created, it’s time for a stable release! The
`stable` branch is produced from the `beta` branch:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Hooray! Rust 1.5 is done! However, we’ve forgotten one thing: because the six
weeks have gone by, we also need a new beta of the *next* version of Rust, 1.6.
So after `stable` branches off of `beta`, the next version of `beta` branches
off of `nightly` again:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

This is called the “train model” because every six weeks, a release “leaves the
station”, but still has to take a journey through the beta channel before it
arrives as a stable release.

Rust releases every six weeks, like clockwork
* If you know the date of one Rust
release, you can know the date of the next one: it’s six weeks later
* A nice
aspect of having releases scheduled every six weeks is that the next train is
coming soon
* If a feature happens to miss a particular release, there’s no need
to worry: another one is happening in a short time! This helps reduce pressure
to sneak possibly unpolished features in close to the release deadline.

Thanks to this process, you can always check out the next build of Rust and
verify for yourself that it’s easy to upgrade to: if a beta release doesn’t
work as expected, you can report it to the team and get it fixed before the
next stable release happens! Breakage in a beta release is relatively rare, but
`rustc` is still a piece of software, and bugs do exist.

### Maintenance time

* 6 weeks / EACH version
  * | release a NEW stable version,
    * the OLD version reaches its EOL (== End Of Life)
* ONLY support the MOST recent stable version
  
### Unstable Features

There’s one more catch with this release model: unstable features
* Rust uses a
technique called “feature flags” to determine what features are enabled in a
given release
* If a new feature is under active development, it lands on
`master`, and therefore, in nightly, but behind a *feature flag*
* If you, as a
user, wish to try out the work-in-progress feature, you can, but you must be
using a nightly release of Rust and annotate your source code with the
appropriate flag to opt in.

If you’re using a beta or stable release of Rust, you can’t use any feature
flags
* This is the key that allows us to get practical use with new features
before we declare them stable forever
* Those who wish to opt into the bleeding
edge can do so, and those who want a rock-solid experience can stick with
stable and know that their code won’t break
* Stability without stagnation.

This book only contains information about stable features, as in-progress
features are still changing, and surely they’ll be different between when this
book was written and when they get enabled in stable builds
* You can find
documentation for nightly-only features online.

### Rustup and the Role of Rust Nightly

Rustup makes it easy to change between different release channels of Rust, on a
global or per-project basis
* By default, you’ll have stable Rust installed
* To
install nightly, for example:

```console
$ rustup toolchain install nightly
```

You can see all of the *toolchains* (releases of Rust and associated
components) you have installed with `rustup` as well
* Here’s an example on one
of your authors’ Windows computer:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

As you can see, the stable toolchain is the default
* Most Rust users use stable
most of the time
* You might want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override` in that project’s directory to set the
nightly toolchain as the one `rustup` should use when you’re in that directory:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Now, every time you call `rustc` or `cargo` inside of
*~/projects/needs-nightly*, `rustup` will make sure that you are using nightly
Rust, rather than your default of stable Rust
* This comes in handy when you
have a lot of Rust projects!

### RFC Process & Teams

* [here](https://github.com/dancer1325/rust-rfcs)
