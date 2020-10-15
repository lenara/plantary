//import 'regenerator-runtime/runtime'
import React from 'react'
import { vtypes, ptypes, pnames, harvestPlant } from './utils'
//import { Home } from './Home'
import getConfig from './config'

const nearConfig = getConfig(process.env.NODE_ENV || 'development')

// define Veggie component: holds & renders a single veggie;
// loads meta_URL and updates.

function	HarvestPlantButton(props){
	function handleClick(e) {
		e.preventDefault();
		if (window.walletConnection.isSignedIn()) {
			harvestPlant(props.vid, props.price);
		}
	}

	return (
		<button className="btn btn-primary" href="#" onClick={handleClick} data-dismiss="modal"><i className="fas fa-seedling"></i>Harvest Plant</button>
	)
}

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

	typeName(){
		if (this.props.vtype == vtypes.PLANT) {
			// TODO: i18n
			return pnames.en[this.props.vsubtype];
		} else 
			return "HARVEST TYPE TODO";
	}

	render(){
		let modalId = "v-" + this.props.vid + "-Modal";
		let harvestPrice = (this.props.vtype == vtypes.PLANT) ? 5 : -1; // TODO: look up
		let harvestJsx = harvestPrice ? (
			<> <br/> <br/><em>Harvest fee: {harvestPrice} Ⓝ</em> </>
		) : (
			<></>
		);
		let harvestButton = harvestPrice ? (
			<HarvestPlantButton price={harvestPrice} vid={this.props.vid} />
		) : (
			<></>
		);


		switch (this.props.renderStyle) {
			case "portfolio": 
				return (
					 <div className="col-md-6 col-lg-4 mb-5">
							<div className="portfolio-item mx-auto" data-toggle="modal" data-target={"#" + modalId}>
									<div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
											<div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
									</div><img className="img-fluid" src={this.state.image} alt={this.state.name}/>
							</div>
						</div>
				);

			case "modal":
				return (
					<div className="portfolio-modal modal fade" id={modalId} tabIndex="-1" role="dialog" aria-labelledby={modalId + "Label"} aria-hidden="true">
						<div className="modal-dialog modal-xl" role="document">
							<div className="modal-content">
								<button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
								<div className="modal-body text-center">
									<div className="container">
										<div className="row justify-content-center">
											<div className="col-lg-8">
												{/* Portfolio Modal - Title */}
												<h2 className="portfolio-modal-title text-secondary mb-0">{this.state.name}</h2>
												{/* Icon Divider */}
												<div className="divider-custom">
													<div className="divider-custom-line"></div>
													<div className="divider-custom-icon"><i className="fas fa-star"></i></div>
													<div className="divider-custom-line"></div>
												</div>
												{/* Portfolio Modal - Image */}<img className="img-fluid rounded mb-5 h-30 w-50" src={this.state.image} alt={this.typeName()}/>
												{/* Portfolio Modal - Text */}
												<p className="mb-5">{this.state.description}
													<br/> <br/><em>Artist: {this.state.artist}</em>
													<br/> <em>Type: {this.typeName()}</em>
													{harvestJsx}
												</p>
												{harvestButton}
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				);

			default: 
				return (
					<div className="veggie">
						<div className="image"><img src={this.state.image}/></div>
						<div className="name">{this.state.name}</div>
						<div className="description">{this.state.description}</div>
						<div className="artist">{this.state.artist}</div>
					</div>
				);
		}
	}

}
// define Veggies component: holds a list of veggie data,
// instantiates individual Veggie components

export class Veggies extends React.Component {
	static defaultProps = {
		vtype: 0, // all veggies
		pageSize: 0, // all veggies
		page: 0 // all veggies
	}

	constructor(props) {
		super(props);
		this.state = {
			// the list of veggies we're loading:
			//vlist: props.vlist || new Array(),
			vlist: new Array(),
		};
		// necessary?
		this.getVeggiesList= this.getVeggiesList.bind(this);
	}

	getVeggiesList(count) {
		let account = window.walletConnection.account();
		if (window.walletConnection.isSignedIn()) {
			window.contract.get_owner_veggies_page_json({ owner_id: window.accountId, vtype: this.props.vtype, page_size: this.props.pageSize, page: this.props.page  })
			.then(vlist => {
				this.setState({vlist: vlist});
			})
		}
			// TODO: handle err
	}

	componentDidMount() {
		this.getVeggiesList();
	}

	render() {
		let vegs = this.state.vlist.map((value, idx) => {
				return (
						<Veggie 
							key={idx}
							vid={value.vid}
							vtype={value.vtype}
							vsubtype={value.vsubtype}
							parent={value.parent_id}
							dna={value.dna}
							meta_url={value.meta_url}
							renderStyle={this.props.renderStyle}
						/>
				)
		});

		return (
			<>
				{vegs}
			</>
		)
	}

}

