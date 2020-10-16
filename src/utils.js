import { connect, Contract, keyStores, WalletConnection, utils, account } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig(process.env.NODE_ENV || 'development')

export const vtypes = {
	PLANT: 1,
	HARVEST: 2
}

export const ptypes = {
	ORACLE: 1,
	PORTRAIT: 2,
	MONEY: 3,
	COMPLIMENT: 4,
	INSULT: 5,
	SEED: 6
}

export const pnames = {
	"en": [
		'',
		'Oracle Plant',
		'Portrait Plant',
		'Money Plant',
		'Compliment Plant',
		'Insult Plant',
		'Seed Plant'
	]
}

export const pprices = [
	-1,
	10,
	20,
	30,
	-1,
	-1,
	-1,
]

export const hprices = [
	-1,
	5,
	5,
	-1,
	5,
	5,
	50
]

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near)

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
		// viewMethods: ['get_owner_veggies_page_json'],
		// some debugging stuff added:
		viewMethods: ['get_owner_veggies_page_json','get_tokens_page','get_veggies_page','get_veggie_json'],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['mint_plant_json','harvest_plant_json'],
  })
}

export function logout() {
  window.walletConnection.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName)
}

export function mintPlant(ptype, price) {
	let amount = utils.format.parseNearAmount(price.toString());
	let account = window.walletConnection.account();
	account.functionCall(nearConfig.contractName, 'mint_plant_json', {
		vsubtype: ptype
	}, 0, amount);
}

export function harvestPlant(parent_id, price) {
	let amount = utils.format.parseNearAmount(price.toString());
	let account = window.walletConnection.account();
	account.functionCall(nearConfig.contractName, 'harvest_plant_json', {
		parent_id: parent_id.toString()
	}, 0, amount);
}
