    bin/devbin infra show|check [--profile <path>]
    bin/devbin infra init [--name <estate>] <domain>... > infra.yaml
    bin/devbin infra profiles <dir>
    bin/devbin infra mail dmarc parse <report|dir> | bin/devbin infra mail dmarc report

Write a profile describing what your estate's DNS and mail should be, and `infra` tells you where reality differs from it. devbin ships no estate of its own and no default profile: a profile is data you supply, found by argument rather than at one fixed place, so it can live inside the project, outside it, or be handed in by CI.

    init      measure live domains and print a profile to review
    show      print where the profile was found, its name and its domains
    check     verify live DNS and mail records against the profile
    profiles  compare every profile under a directory, refusing a domain
              claimed twice
    mail      DMARC aggregate reports: parse them to JSON rows, then
              summarise them

`profiles` searches three levels below `<dir>`, so `<dir>/profiles/<name>/infra.yaml` is found, and a directory holding none is an error rather than a clean estate. So is a directory beneath it that the search cannot read, named by `find`'s own error, or can list but not search, as at mode 0644, named as `cannot search <dir> -- nothing beneath it was looked at`, since a profile there would be compared with nothing. It validates each profile as `show` does, and the first that fails stops the run, named. Each verb takes `--help`.

## Where the profile is found

    --profile <path>                given after the verb
    $DEVBIN_INFRA_PROFILE
    <project root>/infra.yaml
    ~/.config/devbin/infra.yaml

A profile named by `--profile` or by `$DEVBIN_INFRA_PROFILE` is used or refused on that file alone: a missing one is never passed over for the next place in the order. Otherwise the first of the other two that exists is used, and finding none is a hard failure naming every path tried -- never a default, never an empty estate. `show` and `check` both refuse a malformed profile before any record is read. That includes a key the profile has no field for, or one written where nothing reads it, like `dmarc: none` under `defaults:`, and a key written with no value: each is named, with the file, rather than parsed and never read.

## The schema is a file, and you already have it

`infra.reference.yaml` is the annotated worked example: the schema, the search order, the `unreviewed:` rules and the reasoning for each. It ships in devbin's runtime beside `config.reference.yaml`, and `bin/devbin devbin` names the directory that holds them. It is devbin's file, so copy it rather than editing it: doctor flags a vendored file edited in place, and the edit is lost on the next `bin/devbin upgrade --force`. It is not restated here, because two copies of a schema is the drift this command exists to detect.

## Why a declared target rather than a linter

Every record in an estate is correct against the advice in force on the day it was written. Providers revise that advice, domains set up on different dates freeze different snapshots of it, and nothing compares them to each other, so the drift is invisible by construction: there is no moment of carelessness to find. A declared target gives every record something to be compared against.

The sharpest case is a sender that SPF authorises with no DKIM record to match: that domain's own legitimate mail is then indistinguishable from a forgery of it, and nothing says so until the first message flows. `check` reports it as drift, and a profile that declares a sender without both halves of its pair is refused before any record is read. A sender may publish more than one DKIM record, and then every one it declares is half of the pair: each missing one is its own drift, named.

## What check reads

For each domain, from its own nameserver: the MX records when `mail:` is declared; the SPF record, each declared sender's SPF include, and every DKIM record that sender declares; the `_dmarc` record's `p=`, `sp=` and `rua=` against the declared policy, subdomain policy and report address, with an absent `sp=` compared as the `p=` it follows; and the declared wildcard records, `A`, `AAAA` and `TXT` at `*.<domain>`, where `none` is an absence that is checked and a CNAME there contradicts it. The TTL is checked on the MX, SPF, DMARC and wildcard records. Nothing is read for a field the profile does not declare, and a DKIM selector cannot be listed from DNS, so a sender declared with fewer records than it publishes is not something `check` can see.

## Every check must be able to go red

The failures infra is built against are controls that could only ever report success: absent reporting that reads as clean reporting, a cached TTL that reads as a configured one, a DMARC policy that delivers a forged message and merely reports it. So `check` reads each domain's records from its own nameservers, and needs `dig` to do it: there is deliberately no fallback to the system resolver, whose cached TTL would read as the configured one.

It keeps what it measured apart from what it could not:

    held          measured, and it matches
    DRIFT         measured, and it differs -- the estate is wrong
    NOT MEASURED  the instrument failed, and nothing is known
    blind spot    cannot be judged here, and is named on every run

A domain whose nameservers cannot be found is one unmeasured item, never a list of missing records. Finding them is the one question put to the machine's resolver rather than to the domain's own nameservers: an NS query, which `dig` repeats by itself only when no reply came. So when the answer holds no NS record the question is asked again, up to three attempts, `DEVBIN_INFRA_DNS_RETRY_PAUSE` seconds apart (1 by default), and the domain is unmeasured only when every attempt came back without one. A query that got no reply is not asked again, since `dig` has already retried it, and neither is one `dig` could not run at all, since nothing about that changes in a second. The unmeasured line then says what each attempt got -- the status of the reply, `no status` for a reply without one, `no reply` where `dig` heard nothing within its budget, or `dig exit <n>` and the first line of its error where `dig` could not run the query -- and that devbin asked no authoritative nameserver. The statuses tell a resolver's failure from its answer: a SERVFAIL is cached briefly and may clear with a longer `DEVBIN_INFRA_DNS_RETRY_PAUSE`, while an empty NOERROR or an NXDOMAIN is a negative answer the resolver may keep for minutes. `init` records the same sentence under `unreviewed:`. `DEVBIN_INFRA_DNS_TIMEOUT`, `DEVBIN_INFRA_DNS_TRIES` and `DEVBIN_INFRA_DNS_RETRY_PAUSE` must each be a whole number below 100000, and an empty one takes the default; `check` and `init` refuse any other value before a domain is asked about.

Entries under `manual:` are different on purpose: free text for what should be true and can never be verified, never checked and named on every run, so that nobody reads the block as a record of verification.

`check` exits:

    0   every assertion held, and everything was measured
    1   something could not be measured, or drift -- the report names
        the unmeasured first; also every refusal made before a record
        is read -- no profile, a malformed one, an unreviewed
        deviation, or no dig

## Every run states its own blind spots, green or red

A green `check` over DNS reads as "infra is fine" while meaning "the DNS third of infra is fine" -- a summary of what the tool did, mistaken for a statement about the estate. So every run ends by naming what it did not verify, even when it is green, and closes by saying it covered DNS and mail records only:

    not machine-checkable (<n> item(s)) -- declared intent, NEVER verified:
    ...
    blind spots in this run (<n>):
    ...

The blind spots include a DMARC policy that enforces nothing, the same for an explicit subdomain policy (`sp=`), a domain declared `dmarc: none`, and a comparison over a population of one. devbin ships no vendor table, so `mail:` asserts no particular MX value; instead every domain declaring the same `mail:` value must carry the same MX form, and a single such domain has nothing to be compared with, which is reported rather than passed. Documentation cannot do this job, because nobody reads documentation at check time.

## unreviewed: is a gate, not a warning

`init` measures the named domains and prints a proposed profile on stdout -- it writes no file, so redirect it yourself. A value reaches the target only when an RFC or the estate's own majority determines it, and a majority needs more than two domains; every other live value goes under `unreviewed:`. `check` refuses to run, exit 1, while that block holds any entry: accepting a deviation means deleting its key, and fixing it means editing the target. `init` exits 1 when a domain could not be read, still printing the profile, and records that under `unreviewed:` as well. It measures MX, the SPF includes, the `_dmarc` record's `p=`, `sp=` and `rua=`, and the TTL. It proposes no subdomain policy: an `sp=` that differs from its own record's `p=` goes under `unreviewed:`. It measures neither wildcards nor DKIM, which are declared by hand.

## There is no apply

`infra` reads and reports: no verb changes a record or writes the profile, and `check` covers DNS and mail records only.

## Mail

`mail dmarc parse` turns DMARC aggregate reports into one JSON object per record on stdout: a `.xml`, `.xml.gz` or `.zip` file, recognised by its bytes rather than its name, or a directory of them. A directory it cannot list whole is refused and nothing is parsed, since a report beneath what `find` could not read would be missing from the counts; `find`'s own error names it, or, for one it can list but not search, `cannot search <dir> -- nothing beneath it was looked at`. `mail dmarc report` reads those rows on stdin and summarises them for a person. Neither reads a profile or makes a network call. `parse` needs `xmllint` and `jq`, plus `unzip` or `gunzip` for a compressed report, and `report` needs `jq`.

`report` sorts the records into three sections by their DKIM results: no DKIM result at all; DKIM present but not passing; and a signature that passed. The middle one is headed "own mail broken in transit, or a forged signature", because a signature that is present and failing is one or the other and aggregate data cannot say which. Every row with a signature prints each signature's result, its signing domain (`d=`) and its selector (`s=`), where an empty one reads `none` and a selector the report left out reads `absent`, so a signature can be compared with the selectors the domain publishes. A section headed WHAT THIS CANNOT SEE follows, naming what the reports cannot show: a domain that publishes no `rua`, a receiver that sends no reports, message content, whether a failing signature is your own mail or forged, and whether a passing signature belongs to a sender you authorised.

## See also

    bin/devbin devbin                names the directory holding infra.reference.yaml
    bin/devbin infra <verb> --help   each verb's own usage
