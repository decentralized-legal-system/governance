# Requirements of the Governance Layer

## WHAT IS LEGALLY BINDING?

There are two main ways to make our governance layer legally binding (the second I only recently realized):

1) By making everybody participating in the DAO an automatic party to the agreement as described in the whitepaper (
   consensus jurisdiction).

2) By having the DAO create a set of guiding principles, which individual contracting parties agree to (guiding
   principles).

*3) I guess you could think of a third way where a DAO (or NGO/GOVERNMENT) is run by an inner layer of people, and
anybody who wants agree to this jurisdiction does so just as you click to agree when you accept Facebook's T&C, but let
us leave that idea for now.

Regardless of choosing 1 or 2, what is done next on the contract layer must make reference to both the contract, and the
governing laws. Together, two legal documents constitute one agreement binding specific parties. In case of a dispute,
the combination of the two documents in their totality are brought before a court under a more overarching official
governing law (English law, UNUDROIT). The actual consensus that makes it binding can happen at the governance layer (
consensus jurisdiction), or at the contract layer (regular agreement). Obviously, this question matters for the design.

Option 2 would make more sense for our project, and use-cases without a DAO. Option 1 (consensus jurisdiction)
would be the more innovative an inspiring tool. Use cases beyond this project depend on the
industry. As comparison, UPWORK determines the exact details of millions of contracts. OPEC, sets guiding principles,
but the contracts are determined by the individual parties. To make sure our open source package covers both, it would
be best to build the consensus jurisdiction which is accepted and amended by a DAO, and at the same time allow for the
same technology be used for just setting principles (letting go of the consensus aspect). How to do this?

When looking at existing "decentralized" projects, there is a central party creating a T&C. I realized, that for example
the Bored Ape Yacht Club, you are a member of the club by nature of having an NFT. Having the NFT, makes you subject to
a certain number of rights and obligations regardless if you want or not. (And without them, you just have digits in
your wallet). To make this process decentralized, is to have people create their own T&Cs that they are also binding to
all those voting and agreeing for. In short: all we need is a decentralized T&C you can click on a link to read (which
is possible on the IC).

I also realized, that a functionality that can create and maintain a binding T&C, can also create a set of non-binding
principles; a piece of paper can contain a contract as well as a declaration of independence. The end product remains a
DAO managed Wiki which is stored decentralized and can be called via a browser via HTTP. The technique is the same, the
words are just different.

So both options can be addressed with one foundational design:

Create a canister, available through HTTPS, where a DAO is uploaded and laws can be posted and amended. The laws can
then be drafted and amended to either be:

1) Decentralized T&Cs that bind all that participate in a DAO.
2) Guiding principles that bind anyone who so chooses to.

Contracts under 1) bind DAO participants who wish to engage with each other under their general agreement. And example
how this could technically work is a specific contract is called, it verifies that both parties own a specific NFT or
participated in a DAO the last year, produces a timestamp that signifies the agreement, which probably requires add some
centralized details such as name or address (this is not even needed when you use a decentralized court system such as
Aragon court).

Contracts under 2) bind two random parties and make reference to the principles to create one set of binding laws. This
is more like a regular contract.

Perhaps we can go for design 2, because we do not need consensus jurisdiction functionality, but if we design it like
this it becomes more useful in the long run, when the need for a consensus jurisdiction arise.

## AMENDMENTS:

Amendments to a governing law would require a distinct processes:

1) Publication
2) Further amendments.

It would make sense to have a first version of the governance law created with the uploading of the law. But I guess
DAOs should also be able to create additional/side laws for the governance of novel projects or if they wish to vote on
the first version before launching. Laws are then amended according to a set of parameters also established at the
launch of the DAO. Voting is done similarly as currently being done for regular DAO proposals, with more invasive
amendments requiring a larger number of the votes. For example:

* Changes to parameters >50%
* Changes to restrained aspects of the contact >70%
* Changes to text in a contract >80%
* Changes to the DAO itself or these rules >90%

## VERSION CONTROL:

One of the fundamental differences between law and blockchain is that law is flexible and changes over years, and smart
contracts are not. One issue with a DAO amendable governance layer is that the law can change, and thus the guiding
principles. From a legal and technical point of view, it is probably best if laws do not change radically, because it
could result in changes for which the underlying infrastructure is not build. This could be solved with a simple version
control system, were the laws at the time agreement are considered fixed for the duration of the contract. When laws are
amended, the old ones still can be found. The other option could be to not allow amendments outside a few specific
parameters.

## DURATION OF THE CONTRACT:

Speaking about that, it probably makes sense to keep in mind that most legal contracts have end times, while as smart
contracts do not unless you code them.

## WHAT DOES THE GOVERNANCE LAYER STIPULATE?

The governing layer performs two main functions:

1) Sets out the general principles of the agreement (governing laws, details of the agreement, considerations of the
   parties),
2) Sets out parameters to be enforced.

It can even set like industry wide standards. In this example this would be: it sets out that volatility is a risk
factor that cannot be greater than x. At the contract layer, the parties can then agree what level of volatility is
acceptable, or should be less (it cannot be more, because those boundaries have already been set).
