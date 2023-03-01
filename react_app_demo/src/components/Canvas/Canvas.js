import React from 'react';
import img_src from './daisies.jpg';

class Canvas extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      count: 0,
      loadedWasm: false,
      isLoaded: false,
      qrcode: null,
      img: null
    };
  }
  componentDidMount() {
    this.loadWasm();
  }

  drawOriginalImage = async () => {
    const img = new Image();

    img.onload = () => {
      this.img = img;
      const canvas = this.refs.canvas;
      canvas.width = img.width;
      canvas.height = img.height;
      const ctx = canvas.getContext("2d");
      
      ctx.drawImage(img, 0, 0);

    }
    img.src = img_src;
  }

  loadWasm = async () => {

    try {
      const qrcode = await import('qrcode-wasm-jd');

      this.qrcode = qrcode;

      this.drawOriginalImage();

    } finally {
      console.log("loaded wasm successfully");
      this.loadedWasm = true;
      console.log("this.loadedWasm is", this.loadedWasm);
    }

  }
  
  alterChannel = async (channel_index) => {
    const canvas1 = this.refs.canvas;
    const ctx = canvas1.getContext("2d");
    
    ctx.drawImage(this.img, 0, 0);

    let qrcode = this.qrcode;

    console.time("PHOTON_ALTER_CHANNEL");
    // Convert the canvas and context to a PhotonImage
    let image = qrcode.open_image_pass(canvas1, ctx);

    // Filter the image
    qrcode.alter_channel_pass(image, channel_index, 50);
    console.timeEnd("PHOTON_ALTER_CHANNEL");

    // Replace the current canvas' ImageData with the new image's ImageData.
    qrcode.putImageData_pass(canvas1, ctx, image);

  }

  effectPipeline = async() => {
    const canvas1 = this.refs.canvas;
    const ctx = canvas1.getContext("2d");
    
    ctx.drawImage(this.img, 0, 0);

    let qrcode = this.qrcode;
    let phtimg = qrcode.open_image_pass(canvas1, ctx);

    console.time("PHOTON_WITH_RAWPIX");
    qrcode.alter_channel_pass(phtimg, 2, 70);
    qrcode.grayscale_pass(phtimg);
    console.timeEnd("PHOTON_WITH_RAWPIX");

    // // Replace the current canvas' ImageData with the new image's ImageData.
    qrcode.putImageData_pass(canvas1, ctx, phtimg);
  }
  
  render() {
    return(
      <div className="default">

        <div className="sidebar">
            <h3 className="logo">Photon</h3>

            <ul>
              <h4>Channels</h4>
              <li id="alter_red" onClick={() => this.alterChannel(0)}>Increase Red Channel</li>
              <li id="alter_green" onClick={() => this.alterChannel(1)}>Increase Green Channel</li>
              <li id="alter_blue" onClick={() => this.alterChannel(2)}>Increase Blue Channel</li>

              <li id="alter_blue" onClick={this.effectPipeline}>Inc Channel + Threshold</li>

            </ul>     
          </div>

          
          <div className="main">
            <div className="main_content">
   
              <section className="content">
                  <h2>Image</h2>
                  <canvas ref="canvas" />
              </section>

              <section className="benchmarks">
                <div id="time"></div>
                <div id="code"></div>
              </section>
          
          </div>
          
          </div>
          
      </div>
    )
  }
}

export default Canvas