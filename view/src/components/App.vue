<template>
  <canvas id="canvas" :width="width" :height="height"/>
</template>

<script>
import png from '../chen.png'

export default {
  data() {
    return {
      src: null,
      canvasCtx: null,
      width: 460,
      height: 512,
      img: new Image(),
      row: 440,
      col: 512,
      offsetCol: 12,
      limit: 8,
      move: 0,
      ax: 1,
      mode: 0,
      time: 0.
    }
  },

  mounted() {
    this.initCanvas()
    this.loadImage(png)
    this.update()
  },

  methods: {
    initCanvas: function() {
      const canvas = document.getElementById("canvas")
      this.canvasCtx = canvas.getContext("2d")
      this.canvasCtx.clearRect(0, 0, this.width, this.height)
    },

    loadImage: function(file) {
      this.img = new Image()
      this.src = file
      this.img.src = this.src
    },

    render: function() {
      if (this.src != null) {
        this.canvasCtx.clearRect(0, 0, this.width, this.height)
        this.canvasCtx.drawImage(
          this.img,
          this.row * this.step,
          this.col*Number(this.mode),
          this.row,
          this.col,
          this.move,
          0,
          this.row,
          this.col
        )
      }
    },

    getter: function() {
      if (this.src != null) {
        this.mode = Number(external.invoke("getMode"))
        this.time = Number(external.invoke("getTime"))
      }
    },

    update: function() {
      const self = this
      this.intervalid1 = setInterval(function() {
        self.getter()
        self.render()
      }, 1)
    }
  },

  computed: {
    clock: function() {
      return Math.floor(Number(this.time)*4*10)
    },

    step: function() {
      return Math.floor((this.clock % 80) / 10)
    }
  },

  watch: {
    clock: function(val) {
      if (val == 0) {
        this.step = 0
      }
      this.render()
    }
  }
}
</script>
