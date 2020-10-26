# plantary
Grow plants and harvest NFTs

**Demo video for ETHOnline:** https://www.youtube.com/watch?v=qVaP0gPcc14

**Live Demo:** http://plantary.y0b.link/

![](https://i.imgur.com/i0avWYf.png)


**Grow plants and harvest NFTs.**

Plantary is a game and art collecting platform mixed together. 


**Each plant is a NFT and features commissioned artwork from different artists.**

Plants have a unique DNA which influences the rarity and other traits in their harvests. 

**Harvests are also NFTs that can be sold or traded.**


- Mint plants and collect art.

- Combine your plants DNA to mint harvests.

- Collect, sell and trade your NFT harvests.

Built on the NEAR blockchain with Rust


How to Grow and Harvest NFTs
-

The first step is to mint a plant from the Plantary. You can mint a random plant or you can choose a specific type of plant to mint. 

Plant types
-

Possible plants and their harvests include:

- Oracle Plant (get a fortune cookie whenever you need)
- Portrait Plant (get unique AI generated portraits)
- Money Plant (get to brag about owning a money plant)

- Advice Plant (receive advice)
- Compliment Plant (feel good compliments)
- Abstract Art Plant (AI generated abstract art)

- Seed Plant (get more plants)
- Insult Plant (creative insults)
- Nature Art Plant (AI generated nature pictures)

- Poem plant
- Bad Advice Plant
- Lucky Numbers Plant 

- Ideas Plant
- Trivia Plant
- Leafy Plant 

- Superpower Plant
- Psychodelic Art Plant
- Meaning of Life Plant


Minting a plant
-

Each plant has different minting fees. For example:

- random plant: 10 Ⓝ
- money plant: 15 Ⓝ
- oracle plant: 20 Ⓝ
- portrait plant: 25 Ⓝ

Plant DNA
-

When a plant is minted it will have a type and also a unique DNA string, which will have an influence on what the plant can do. 


Harvests
-

Plants can give harvests. The type of plant defines the kind of harvest. The portrait plant allows you to mint portaits. The oracle plant allows you to mint fortunes. You need to own at least one oracle plant to be able to mint fortunes, and at least one portrait plant in order to mint portraits. The money plant does not give harvests.

Harvest mints carry a fee depending on the type of harvest. Example:

- each fortune harvest: 5 Ⓝ
- each portrait harvest: 10 Ⓝ

The DNA of the plants you own influence the types of harvest you can get (rarity and other qualities). You can select from the plants you have which ones you want to use to influence each harvest mint.

Rarity
-

Harvests have different levels or rarity. Example:

- standard harvest - common
- special harvest - rare
- power harvest - ultra rare
- unique harvest - legendary
- magic harvest - mythic

Some plants and combination of plants will produce a higher amount of rare harvest mints.

Sell and trade NFTs
-

Both plants and harvests are unique NFTs that can be sold or traded. 


Collect Art
-

The plant NFT images will feature commissioned art from different artists. Plantary will open submissions for artists to send plant artwork. After submissions are approved, artists get a commission on first sales, and royalties on further secondary market sales.

Artist & Curator Interface
-

Artists submissions are converted to NFT metadata genes and persisted on the arweave permaweb.  
Curators can then approve, price and publish new NFT options from these gene pools.

Deployment & Testing 
-

You can deploy Plantary in your own testnet account. 

* Check out the git repo, or open it in gitpod.  

(If you are using GitPod, you can sit back and watch while the next three steps happen automatically in your console.  Then skip down to "Log in to the testnet", below.)

* Install all dependencies:
```
    yarn install
    rustup target add wasm32-unknown-unknown
```
* Add the NEAR CLI to your searchpath.  

The NEAR CLI should have been installed by 'yarn install'.  
Here is the unix command to add it to your search path:
```    
    export PATH="./node_modules/near-cli/bin":$PATH
```
(Windows users do some other thing.)
* Build and test the project:
```
    yarn build
    yarn test
```

* Log in to the testnet with the NEAR CLI
```
    near login
```
  ... and click the link provided to open a web browser and authorize the CLI with your testnet account.

* Start the app!
```
		yarn start
```
This will deploy the contract on the testnet, build the UI, start a test webserver, and open a web browser.
(Don't mint a plant yet, there's still one more step!)

In the console you'll see a line something like this:
```
Starting deployment. Account id: dev-1602291649659-5495547, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./out/main.wasm
```
Take note of that Account id: dev-numbersnumbersnumbers.  You can use that for direct interaction with the contract you deployed, via the NEAR CLI.

* Initialize the smart contract!

The very first time your contract is deployed in a testnet account, it must be initialized.  Open a new terminal window and enter these commands, replacing YOURACCOUNTID with your account ID on the NEAR testnet, and YOURCONTRACTID with the Account ID from the previous step:
```
    export PATH="./node_modules/near-cli/bin":$PATH
    near call --accountId YOURACCOUNTID TESTCONTRACTID new '{"owner_id": "YOURACCOUNTID"}'
```
* Now you can use the Plantary web app to connect your wallet, mint plants and gather harvests!

* Disclaimer: don't get attached to your NFTs, this is the testnet, we are still working on the contract and they can disappear at any tim.

* Genome interface (Work In Progress)

While the test server is still running, open http://localhost:1234/intake.html to use the arweave intake form. 
