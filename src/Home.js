import React from 'react'
import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import { login, logout, mintPlant, vtypes, ptypes } from './utils'
import { Veggies } from './Veggies'
import getConfig from './config'

const nearConfig = getConfig(process.env.NODE_ENV || 'development')

function AccountOrWallet() {
	if (window.walletConnection.isSignedIn()) 
		return window.walletConnection.getAccountId();
	else
		return "CONNECT WALLET";
}

function MineOrWallet() {
	if (window.walletConnection.isSignedIn()) 
		return "MY PLANTARY";
	else
		return "CONNECT WALLET";
}

function WalletLink() {
  function handleClick(e) {
    e.preventDefault();
		if (!window.walletConnection.isSignedIn()) {
			login();
		} else {
			logout();
		}
  }

	let faIcon = window.walletConnection.isSignedIn() ? "fa-sign-out-alt" : "fa-cog";

  return (
		<a href="#" className="btn btn-outline-light btn-social mx-1" onClick={handleClick} ><i className={'fas ' + faIcon}></i></a>
  );
}

function MintPlantButton(props){
  function handleClick(e) {
    e.preventDefault();
		if (window.walletConnection.isSignedIn()) {
			mintPlant(props.pType, props.price);
		} else {
			login();
		}
	}

	return (
		<button className="btn btn-primary" href="#" onClick={handleClick} data-dismiss="modal"><i className="fas fa-seedling"></i> Mint Plant</button>
	)
}

export function Home() {
	return (
		<>
        <nav className="navbar navbar-expand-lg bg-secondary fixed-top" id="mainNav">
            <div className="container"><a className="navbar-brand js-scroll-trigger" href="#page-top">PLANTARY</a>
                <button className="navbar-toggler navbar-toggler-right font-weight-bold bg-primary text-white rounded" type="button" data-toggle="collapse" data-target="#navbarResponsive" aria-controls="navbarResponsive" aria-expanded="false" aria-label="Toggle navigation">Menu <i className="fas fa-bars"></i></button>
                <div className="collapse navbar-collapse" id="navbarResponsive">
                    <ul className="navbar-nav ml-auto">
                        <li className="nav-item mx-0 mx-lg-1"><a className="nav-link py-3 px-0 px-lg-3 rounded js-scroll-trigger" href="#portfolio">MINT A PLANT</a>
                        </li>
                        <li className="nav-item mx-0 mx-lg-1"><a className="nav-link py-3 px-0 px-lg-3 rounded js-scroll-trigger" href="#about">HARVEST</a>
                        </li>
                        <li className="nav-item mx-0 mx-lg-1"><a className="nav-link py-3 px-0 px-lg-3 rounded js-scroll-trigger" href="#contact">ABOUT</a>
                        </li>
												<li className="nav-item mx-0 mx-lg-1"><a className="nav-link py-3 px-0 px-lg-3 rounded js-scroll-trigger" href="#connect"><MineOrWallet /></a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
        <header className="masthead bg-primary text-white text-center">
            <div className="container d-flex align-items-center flex-column">
							{/* Masthead Avatar Image*/}<img className="masthead-avatar mb-5" src={require("./assets/img/plantaryapp.png")} alt=""/>
								{/* Masthead Heading*/}
                <h1 className="masthead-heading mb-0">Grow and Harvest NFTs</h1>
								{/* Icon Divider*/}
                <div className="divider-custom divider-light">
                    <div className="divider-custom-line"></div>
                    <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                    <div className="divider-custom-line"></div>
                </div>
								{/* Masthead Subheading*/}
                <p className="pre-wrap masthead-subheading font-weight-light mb-0">Start by minting a plant using the NEAR NFT contract</p>
            </div>
        </header>
        <section className="page-section portfolio" id="portfolio">
            <div className="container">
							{/* Portfolio Section Heading*/}
                <div className="text-center">
                    <h2 className="page-section-heading text-secondary mb-0 d-inline-block">CHOOSE A PLANT</h2>
                </div>
								{/* Icon Divider*/}
                <div className="divider-custom">
                    <div className="divider-custom-line"></div>
                    <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                    <div className="divider-custom-line"></div>
                </div>
								{/* Portfolio Grid Items*/}
                <div className="row justify-content-center">
									{/* Portfolio Items*/}
                    <div className="col-md-6 col-lg-4 mb-5">
                        <div className="portfolio-item mx-auto" data-toggle="modal" data-target="#portfolioModal0">
                            <div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
                                <div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
															</div><img className="img-fluid" src={require("./assets/img/portfolio/plant1.png")} alt="Oracle Plant"/>
                        </div>
                    </div>
                    <div className="col-md-6 col-lg-4 mb-5">
                        <div className="portfolio-item mx-auto" data-toggle="modal" data-target="#portfolioModal1">
                            <div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
                                <div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
															</div><img className="img-fluid" src={require("./assets/img/portfolio/plant2.png")} alt="Money Plant"/>
                        </div>
                    </div>
                    <div className="col-md-6 col-lg-4 mb-5">
                        <div className="portfolio-item mx-auto" data-toggle="modal" data-target="#portfolioModal2">
                            <div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
                                <div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
															</div><img className="img-fluid" src={require("./assets/img/portfolio/plant3.png")} alt="Portrait Plant"/>
                        </div>
                    </div>
                    <div className="col-md-6 col-lg-4 mb-5">
                        <div className="portfolio-item mx-auto" data-toggle="modal" data-target="#portfolioModal3">
                            <div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
                                <div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
															</div><img className="img-fluid" src={require("./assets/img/portfolio/plant4b.png")} alt="Compliment Plant"/>
                        </div>
                    </div>
                    <div className="col-md-6 col-lg-4 mb-5">
                        <div className="portfolio-item mx-auto" data-toggle="modal" data-target="#portfolioModal4">
                            <div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
                                <div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
															</div><img className="img-fluid" src={require("./assets/img/portfolio/plant6.png")} alt="Insult Plant"/>
                        </div>
                    </div>
                    <div className="col-md-6 col-lg-4 mb-5">
                        <div className="portfolio-item mx-auto" data-toggle="modal" data-target="#portfolioModal5">
                            <div className="portfolio-item-caption d-flex align-items-center justify-content-center h-100 w-100">
                                <div className="portfolio-item-caption-content text-center text-white"><i className="fas fa-plus fa-3x"></i></div>
															</div><img className="img-fluid" src={require("./assets/img/portfolio/plant5.png")} alt="Seed Plant"/>
                        </div>
                    </div>
                </div>
            </div>
        </section>
				{/* Portfolio Modal*/}
        <div className="portfolio-modal modal fade" id="portfolioModal0" tabIndex="-1" role="dialog" aria-labelledby="#portfolioModal0Label" aria-hidden="true">
            <div className="modal-dialog modal-xl" role="document">
                <div className="modal-content">
                    <button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
                    <div className="modal-body text-center">
                        <div className="container">
                            <div className="row justify-content-center">
                                <div className="col-lg-8">
																	{/* Portfolio Modal - Title*/}
                                    <h2 className="portfolio-modal-title text-secondary mb-0">Oracle Plant</h2>
																		{/* Icon Divider*/}
                                    <div className="divider-custom">
                                        <div className="divider-custom-line"></div>
                                        <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                                        <div className="divider-custom-line"></div>
                                    </div>
																		{/* Portfolio Modal - Image*/}<img className="img-fluid rounded mb-5" src={require("./assets/img/portfolio/plant1.png")} alt="Oracle Plant"/>
																		{/* Portfolio Modal - Text*/}
                                    <p className="mb-5">The oracle plant is a mythical being with syncretic wisdom laying dormant in its fruit, waiting for questions to blossom in the seeker's mind.
	                                    <br/> <br/><em>Minting fee: 10 Ⓝ</em>
                                    </p>
																		<MintPlantButton pType={ptypes.ORACLE} price="10" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div className="portfolio-modal modal fade" id="portfolioModal1" tabIndex="-1" role="dialog" aria-labelledby="#portfolioModal1Label" aria-hidden="true">
            <div className="modal-dialog modal-xl" role="document">
                <div className="modal-content">
                    <button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
                    <div className="modal-body text-center">
                        <div className="container">
                            <div className="row justify-content-center">
                                <div className="col-lg-8">
																		{/* Portfolio Modal - Title*/}
                                    <h2 className="portfolio-modal-title text-secondary mb-0">Money Plant</h2>
																		{/* Icon Divider*/}
                                    <div className="divider-custom">
                                        <div className="divider-custom-line"></div>
                                        <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                                        <div className="divider-custom-line"></div>
                                    </div>
																		{/* Portfolio Modal - Image*/}<img className="img-fluid rounded mb-5" src={require("./assets/img/portfolio/plant2.png")} alt="Money Plant"/>
																		{/* Portfolio Modal - Text*/}
                                    <p className="mb-5">You always wished for a money plant and here it is. The mere sight of its wealthy leaves will bring abundance to your life, even if they can't be used to buy groceries.
	                                    <br/> <br/><em>Minting fee: 30 Ⓝ</em>
                                    </p>
																		<MintPlantButton pType={ptypes.MONEY} price="30" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div className="portfolio-modal modal fade" id="portfolioModal2" tabIndex="-1" role="dialog" aria-labelledby="#portfolioModal2Label" aria-hidden="true">
            <div className="modal-dialog modal-xl" role="document">
                <div className="modal-content">
                    <button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
                    <div className="modal-body text-center">
                        <div className="container">
                            <div className="row justify-content-center">
                                <div className="col-lg-8">
																		{/* Portfolio Modal - Title*/}
                                    <h2 className="portfolio-modal-title text-secondary mb-0">Portrait Plant</h2>
																		{/* Icon Divider*/}
                                    <div className="divider-custom">
                                        <div className="divider-custom-line"></div>
                                        <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                                        <div className="divider-custom-line"></div>
                                    </div>
																		{/* Portfolio Modal - Image*/}<img className="img-fluid rounded mb-5" src={require("./assets/img/portfolio/plant3.png")} alt="Portrait Plant"/>
																		{/* Portfolio Modal - Text*/}
                                    <p className="mb-5">The portrait plant ripens a multitude of faces, each with unique features. You might see in their eyes a reflection of a familiar facet, or a glimpse from the unknown.
	                                    <br/> <br/><em>Minting fee: 20 Ⓝ</em>
                                    </p>
																		<MintPlantButton pType={ptypes.PORTRAIT} price="20" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div className="portfolio-modal modal fade" id="portfolioModal3" tabIndex="-1" role="dialog" aria-labelledby="#portfolioModal3Label" aria-hidden="true">
            <div className="modal-dialog modal-xl" role="document">
                <div className="modal-content">
                    <button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
                    <div className="modal-body text-center">
                        <div className="container">
                            <div className="row justify-content-center">
                                <div className="col-lg-8">
																		{/* Portfolio Modal - Title*/}
                                    <h2 className="portfolio-modal-title text-secondary mb-0">Compliment Plant</h2>
																		{/* Icon Divider*/}
                                    <div className="divider-custom">
                                        <div className="divider-custom-line"></div>
                                        <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                                        <div className="divider-custom-line"></div>
                                    </div>
																		{/* Portfolio Modal - Image*/}<img className="img-fluid rounded mb-5" src={require("./assets/img/portfolio/plant4b.png")} alt="Compliment Plant"/>
																		{/* Portfolio Modal - Text*/}
                                    <p className="mb-5">Available soon.</p>
																		{/*<button className="btn btn-primary" href="#" data-dismiss="modal"><i className="fas fa-seedling"></i> Mint Plant</button>*/}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div className="portfolio-modal modal fade" id="portfolioModal4" tabIndex="-1" role="dialog" aria-labelledby="#portfolioModal4Label" aria-hidden="true">
            <div className="modal-dialog modal-xl" role="document">
                <div className="modal-content">
                    <button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
                    <div className="modal-body text-center">
                        <div className="container">
                            <div className="row justify-content-center">
                                <div className="col-lg-8">
																		{/* Portfolio Modal - Title*/}
                                    <h2 className="portfolio-modal-title text-secondary mb-0">Insult Plant</h2>
																		{/* Icon Divider*/}
                                    <div className="divider-custom">
                                        <div className="divider-custom-line"></div>
                                        <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                                        <div className="divider-custom-line"></div>
                                    </div>
																		{/* Portfolio Modal - Image*/}<img className="img-fluid rounded mb-5" src={require("./assets/img/portfolio/plant6.png")} alt="Insult Plant"/>
																		{/* Portfolio Modal - Text*/}
                                    <p className="mb-5">Available soon.</p>
																		{/*<button className="btn btn-primary" href="#" data-dismiss="modal"><i className="fas fa-seedling"></i> Mint Plant</button>*/}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div className="portfolio-modal modal fade" id="portfolioModal5" tabIndex="-1" role="dialog" aria-labelledby="#portfolioModal5Label" aria-hidden="true">
            <div className="modal-dialog modal-xl" role="document">
                <div className="modal-content">
                    <button className="close" type="button" data-dismiss="modal" aria-label="Close"><span aria-hidden="true"><i className="fas fa-times"></i></span></button>
                    <div className="modal-body text-center">
                        <div className="container">
                            <div className="row justify-content-center">
                                <div className="col-lg-8">
																		{/* Portfolio Modal - Title*/}
                                    <h2 className="portfolio-modal-title text-secondary mb-0">Seed Plant</h2>
																		{/* Icon Divider*/}
                                    <div className="divider-custom">
                                        <div className="divider-custom-line"></div>
                                        <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                                        <div className="divider-custom-line"></div>
                                    </div>
																		{/* Portfolio Modal - Image*/}<img className="img-fluid rounded mb-5" src={require("./assets/img/portfolio/plant5.png")} alt="Seed Plant"/>
																		{/* Portfolio Modal - Text*/}
                                    <p className="mb-5">Available soon.</p>
																		{/*<button className="btn btn-primary" href="#" data-dismiss="modal"><i className="fas fa-seedling"></i> Mint Plant</button>*/}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <section className="page-section bg-primary text-white mb-0" id="about">
            <div className="container">
								{/* About Section Heading*/}
                <div className="text-center">
                    <h2 className="page-section-heading d-inline-block text-white">HARVEST</h2>
                </div>
								{/* Icon Divider*/}
                <div className="divider-custom divider-light">
                    <div className="divider-custom-line"></div>
                    <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                    <div className="divider-custom-line"></div>
                </div>
								{/* About Section Content*/}
                <div className="row">
                    <div className="col-lg-4 ml-auto">
                        <p className="pre-wrap lead">After you own a plant, you can use it to mint harvests. The type of plant defines the kind of harvest.

You need to own at least one oracle plant to be able to mint fortunes, and at least one portrait plant in order to mint portraits. The money plant does not give harvests.
</p>
                    </div>
                    <div className="col-lg-4 mr-auto">
                        <p className="pre-wrap lead">The DNA of the plants you own influence the types of harvest you can get (rarity and other qualities). 

Select from the plants you have which ones you want to use to influence each harvest mint. 

THIS SECTION COMING SOON!</p>
                    </div>
                </div>
            </div>
        </section>
        <section className="page-section" id="contact">
            <div className="container">
								{/* Contact Section Heading*/}
                <div className="text-center">
                    <h2 className="page-section-heading text-secondary d-inline-block mb-0">ABOUT PLANTARY</h2>
                </div>
								{/* Icon Divider*/}
                <div className="divider-custom">
                    <div className="divider-custom-line"></div>
                    <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                    <div className="divider-custom-line"></div>
                </div>
								{/* Contact Section Content*/}
                <div className="row justify-content-center">
                    <div className="col-lg-4">
                        <div className="d-flex flex-column align-items-center">
                            <div className="icon-contact mb-3"><i className="fas fa-seedling"></i></div>
                            <div className="text-muted">NFT contract</div>
                            <div className="lead font-weight-bold">Built on NEAR blockchain</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        <section className="page-section connect" id="connect">
        <footer className="footer text-center">
            <div className="container">
                <div className="row">
										{/* Footer Location*/}
                    <div className="col-lg-4 mb-5 mb-lg-0">
                        <h4 className="mb-4"></h4>
												<p className="pre-wrap lead mb-0"></p>
                    </div>
										{/* Footer Social Icons*/}
                    <div className="col-lg-4 mb-5 mb-lg-0">
											<h4 className="mb-4"><AccountOrWallet /></h4><WalletLink />
                    </div>
										{/* Footer About Text*/}
                    <div className="col-lg-4">
                        <h4 className="mb-4"></h4>
												<p className="pre-wrap lead mb-0"></p>
                    </div>
                </div>
            </div>
        </footer>
        </section>

				<section className="page-section portfolio" id="myPlantPorfolio">
            <div className="container">
                {/* Section Heading */}
                <div className="text-center">
                    <h2 className="page-section-heading text-secondary mb-0 d-inline-block">MY PLANTS</h2>
                </div>
                {/* Icon Divider */}
                <div className="divider-custom">
                    <div className="divider-custom-line"></div>
                    <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                    <div className="divider-custom-line"></div>
                </div>
                {/* Portfolio Grid Items */}
                <div className="row justify-content-center">
                    {/* Portfolio Items */}
										<Veggies vtype={vtypes.PLANT} renderStyle="portfolio"/>
                </div>
            </div>
        </section>
				<Veggies vtype={vtypes.PLANT} renderStyle="modal" />

				<section className="page-section portfolio bg-primary text-white mb-0" id="myHarvestPorfolio">
            <div className="container">
              {/* Section Heading */}
                <div className="text-center">
                    <h2 className="page-section-heading d-inline-block text-white">MY HARVESTS</h2>
                </div>
                {/* Icon Divider */}
                <div className="divider-custom divider-light">
                    <div className="divider-custom-line"></div>
                    <div className="divider-custom-icon"><i className="fas fa-star"></i></div>
                    <div className="divider-custom-line"></div>
                </div>
                {/* Portfolio Grid Items */}
                <div className="row justify-content-center">
                    {/* Portfolio Items */}
										<Veggies vtype={vtypes.HARVEST} renderStyle="portfolio"/>
                </div>
            </div>
        </section>
				<Veggies vtype={vtypes.HARVEST} renderStyle="modal" />

				{/* Copyright Section*/}
        <section className="copyright py-4 text-center text-white">
            <div className="container"><small className="pre-wrap">Copyright © Plantary 2020 </small></div>
        </section>
				{/* Scroll to Top Button (Only visible on small and extra-small screen sizes)*/}
        <div className="scroll-to-top d-lg-none position-fixed"><a className="js-scroll-trigger d-block text-center text-white rounded" href="#page-top"><i className="fa fa-chevron-up"></i></a></div>
		</>
	)
}
