# plantary
Grow plants and harvest NFTs

**Demo video:** http://www.lenara.com/plantary/Plantary-demo-video.mp4 

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

Deployment & Testing (10/10/2020)
-

This is a work in progress!  
As of 10/10, you can run a crude mockup of the app -- all it can do is login and logout.
But it's interacting with your near wallet, and deploying a smart contract.

* Check out the git repo, or open it in gitpod

* Install all dependencies:
```
    yarn install
    rustup target add wasm32-unknown-unknown
```
* Build and test the proejct:
```
    yarn build
    yarn test
```
* Start the app!
```
		yarn start
```
This will deploy the contract on the testnet, build the UI, and open a web browser.
In the console you'll see a line something like this:
```
Starting deployment. Account id: dev-1602291649659-5495547, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./out/main.wasm
```
Take note of that Account id: dev-numbersnumbersnumbers.  You can use that for direct interaction with the contract you deployed, via the NEAR CLI.

Below are instructions for manually interacting with the smart contract on the command line.
This will be less important as we get those features into the app.  
But for the time being, here are the NEAR CLI instructions for console use:

* First, get the NEAR CLI in your searchpath.  (Here is the unix/gitpod method; windows users do some other thing ...)
```    
export PATH="/workspace/plantary/node_modules/near-cli/bin/":$PATH
```
* Log in to the testnet with the NEAR CLI
```
    near login
```
  ... and click the link provided to open your wallet to NEAR.

* The very first time your contract runs, it must be initialized like so:
```
    near call --accountId YOURACCOUNTID TESTCONTRACTID new '{"owner_id": "YOURACCOUNTID"}'
```
* Then, to mint a plant in the blockchain, call mint_plant() like so:
```
    near call --accountId YOURACCOUNTID TESTCONTRACTID mint_plant '{"vsubtype": 2}'
```
  If that works, you should see something like this:
```
Scheduling a call: dev-1601436477182-8079303.mint_plant({"vsubtype": 2})
Transaction Id AG5iRNasrPr1FSwxGnngVedkKABNBrKU7Sn6X4rYndu2
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/AG5iRNasrPr1FSwxGnngVedkKABNBrKU7Sn6X4rYndu2
{ id: 0, vtype: 1, vsubtype: 2, parent: 0 }
```

That last bit of JSON is the data returned from the call. Nothing much there yet ... more soon!

* Each new plant minted will have a unique id - they are non-fungible.

* You can retrieve minted tokens by calling get_plant() with the token id. To get the first plant minted use:
```
    near call --accountId YOURACCOUNTID TESTCONTRACTID get_plant '{"vid": 0}'
```
The contract also provides a delete_plant() function if you must.

--- 

The UI has not yet been connected to the contract but can be previewed at: http://www.lenara.com/plantary/


Plantary Team:
-

@lenara 
@ivenka 
@mari-gold 
@myklemykle 
@shad-k 
