//import 'regenerator-runtime/runtime'
import React from 'react'
//import { login, logout, mintPlant } from './utils'
//import { Home } from './Home'
import './global.css'
import getConfig from './config'

const nearConfig = getConfig(process.env.NODE_ENV || 'development')

// define Veggie component: holds & renders a single veggie;
// loads meta_URL and updates.

export class Veggie extends React.Component {
	constructor(props) {
    super(props);
		this.state = {};

		for (var p in [
			// these properties come from the server Veggie object:
			//vid,
			//vtype,
			//vsubtype,
			//parent_vid,
			//dna,
			//meta_url,
			// these will be parsed from the data at the meta_url, if not provided:
			'name',
			'description',
			'image',
			'artist'
		]) {
			this.state[p] = props[p];
		}
		this.getVeggieMeta = this.getVeggieMeta.bind(this);
  }

	componentDidMount() {
    this.getVeggieMeta();
  }

	// The NEAR blockchain stores ownership and a bit more, but some
	// veggie metadata lives elsewhere.  metaURL points to a hunk of
	// JSON on the web where we can load those props.
	getVeggieMeta(){
		$.getJSON(this.props.meta_url)
			.then(obj => {
					const picked = (({ 
							name,
							description,
							image,
						}) => ({ 
							name,
							description,
							image,
						}) )(obj);

					// decompose this "attribues" array of "trait_type"->"value" pairs to dig out the "artist" trait.
					if (obj.attributes && obj.attributes.length) { 
						let artistTrait = obj.attributes.find(t => t.trait_type == "artist");
						picked.artist = artistTrait ? ( "Artist: " + artistTrait.value ): NULL;
					}

					this.setState(picked);
			})
		;
	}

	render(){
		// For now just plants ...
		return (
			<div className="veggie">
				<div className="image"><img src={this.state.image}/></div>
				<div className="name">{this.state.name}</div>
				<div className="description">{this.state.description}</div>
				<div className="artist">{this.state.artist}</div>
			</div>
		)
	}
}
// define Veggies component: holds a list of veggie data,
// instantiates individual Veggie components

export class Veggies extends React.Component {
	constructor(props) {
    super(props);
		this.state = {
			// the list of veggies we're loading:
			//vlist: props.vlist || new Array(),
			vlist: new Array(),
		};
		// necessary?
		this.getPlantsList= this.getPlantsList.bind(this);
  }

	componentDidMount() {
    this.getPlantsList();
  }

	// TODO: the difference btwn plant and harvest is so minor, 
	// we should have just one API for both.  And it should
	// include paging ...
	getPlantsList() {
		let account = window.walletConnection.account();
		if (window.walletConnection.isSignedIn()) {
			window.contract.get_owner_plants({ owner_id: window.accountId })
			.then(vlist => {
				this.setState({vlist: vlist});
			})
		}
			// TODO: handle err
	}

		
  render() {
		let vegs = this.state.vlist.map((value, idx) => {
				return (
					<li key={idx}>
						<Veggie 
							vid={value.vid}
							vtype={value.vtype}
							vsubtype={value.vsubtype}
							parent={value.parent_id}
							dna={value.dna}
							meta_url={value.meta_url}
						/>
					</li>
				)
		});

		return (
			<ul className="veggies">
				{vegs}
			</ul>
		)
  }

}

