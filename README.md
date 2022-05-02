# Spring is NEAR: Challenge 5: GiveWell Donation

For detailed info, check [this video](https://youtu.be/SgVn04hEXBs) about the weaknesses that we won't speak about here (Not repeating myself). 

Let's talk about what's not in the video (plus some repeats for smoother reading): 

## Contract
The contract cloned from [this repo](https://github.com/near-examples/nft-tutorial/tree/main/nft-contract/src) and make changes. Particularly, we refrain users from minting themselves by making the `nft_mint` a private function, and they have to donate and mint an NFT. 
To handle multiple mints in one click of the button, we make a `minting_interface` to repeatedly call `donate_and_mint` or `donate_and_update` (the latter just update the existing NFTs than minting a new one, if already exist for that group) and handle some transactions. 

Extra stuffs on weaknesses, please check second part of the video, where one talks about 4 weaknesses of the contract. 

### How can this be improved? 
Well, "this" refers to our `minting_interface`, not the 4 weaknesses from video. With a `minting_interface`, we are programming from Object-Oriented perspective. However, taking from gaming experiences, this is a use case of [data-oriented design (DOD)](https://www.dataorienteddesign.com/dodbook/). 
In video games, if you have 1000 objects, your GPU might not be strong enough to render the graphics of each individuals, and you either buy an even stronger graphic card, use multiple graphic cards, or 
change the way you look at objects. By throwing away some easiness that object-oriented brings (we humans easily look at object) and mingle things up, we can optimize the computing experiences (i.e. reduce gas cost and storage consumption on-chain). 

Exactly how that can be done is another story. One'll hopefully write about it, either in the wiki of this Github, or my [personal blog](wabinab.github.io), or somewhere else **perhaps after Spring is NEAR 2022 ends, I don't know**. 

## Frontend
For navigation of frontend, check out the video. 


---

## Instruction for cloners.

The first thing is to bundle install stuff. We only want non production. 

```
bundle config set --local without production
bundle install
```

The second thing is to recreate master key and credentials. 

```
EDITOR="code ." bin/rails credentials:edit
```

**Close the file.** Then run migrations:

```
rails db:migrate
```

Then we need to install bootstrap. (Ignore the error, it'll auto install upon cannot find bootstrap). 
This requires yarn and node js. 

```
bash rebuild.sh
```

Everything should be fine after that. Try to start `rails s` and see if it starts or not. 

```
rails s
```

Everything should be fine after that. Try to start `rails s` and see if it starts or not. 

## References
- https://github.com/near-examples/nft-tutorial/tree/main/nft-contract/src
