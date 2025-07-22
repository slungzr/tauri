<!-- 方案相位展示 -->
<template>
	<a-row>
		<canvas :id="'canvas' + id" :style="{ 
			border: '0px solid #ccc', 
			borderRadius: circle ? '50%' : '0', 
			overflow: 'hidden' 
		}" :width="width" :height="height"></canvas>
	</a-row>
</template>

<script>
export default {
	name: 'Phase',
	props: {
		phaseImage: {
			type: Object,
			required: false
		},
		id: {
			type: String,
			default: () => Date.now() + "_" + Math.random(),
			required: false
		},
		crossNo: {
			type: [Number,String],
			required: false
		},
		planNo: {
			type: [String,Number],
			required: false
		},
		stageNo: {
			type: [Number,String],
			required: false
		},
		lineWidth: {
			type: Number,
			default: 2,
			required: false
		},
		duration: {
			type: [Number,String],
			default: 0,
			required: false
		},
		width: {
			type: [Number,String],
			default: 200,
			required: true
		},
		height: {
			type: [Number,String],
			default: 200,
			required: true
		},
		backgroundColor: {
			type: String,
			default: '#000000',
			required: false
		},
		lineColor: {
			type: String,
			default: '#07e463',
			required: false
		},
		showDuration: {
			type: Boolean,
			default: true,
			required: false
		},
		circle: {
			type: Boolean,
			default: false,
			required: false
		}
	},
	data() {
		return {
			canvas: null,
			ctx: null,
			crossInfo: {},
			isUpdating: false
		}
	},
	watch: {
		
		duration(val) {
			if (!this.showDuration) return;
			this.clearCanvas();
			this.drawInfo();
			if (val > 0) {
				this.drawDuration(val);
			}
		},
		showDuration(val) {
			this.clearCanvas();
			this.drawInfo();
			if (val && this.duration > 0) {
				this.drawDuration(this.duration);
			}
		}
	},
	mounted() {
		this.initCanvas();
		this.getCrossInfo();
	},
	methods: {
		initCanvas() {
			this.canvas = document.getElementById('canvas' + this.id);
			this.ctx = this.canvas.getContext('2d');
			this.ctx.fillStyle = this.backgroundColor;
			this.ctx.fillRect(0, 0, this.width, this.height);
		},
		clearCanvas() {
			this.ctx.clearRect(0, 0, this.width, this.height);
			this.ctx.fillStyle = this.backgroundColor;
			this.ctx.fillRect(0, 0, this.width, this.height);
		},
		getCrossInfo() {
			this.crossInfo = this.phaseImage
			this.drawInfo();
			this.isUpdating = false;
		},
		drawInfo() {
			if (!this.crossInfo.phaseList) return;
			this.crossInfo.phaseList.forEach((phase) => {
				this.drawLine(parseInt(phase.dir), phase.mov);
			});

			if(this.duration > 0 && this.showDuration){
				this.drawDuration(this.duration);
			}
		},
		drawDuration(duration) {
			this.ctx.font = '12px Arial';
			this.ctx.fillStyle = this.lineColor;
			this.ctx.textAlign = 'center';
			this.ctx.textBaseline = 'middle';
			this.ctx.fillText(duration, this.width/2, this.height/2);
		},
		drawLine(dir, movs) {
			const ctx = this.ctx;
			let w = this.width;
			let h = this.height;
			const lc = this.lineColor;
			const lw = this.lineWidth;

			ctx.strokeStyle = lc;
			ctx.lineWidth = lw;
			ctx.lineCap = 'round';
			ctx.lineJoin = 'round';

			let scale = 1;
			if (w < 100) {
				scale = 150 / w;
				w = 150;
				h = 150;
			}

			const drawArrow = (x, y, angle) => {
				const arrowLength = 8;
				const arrowAngle = Math.PI / 6;
				
				ctx.beginPath();
				ctx.moveTo(x, y);
				ctx.lineTo(
					x - arrowLength * Math.cos(angle - arrowAngle),
					y - arrowLength * Math.sin(angle - arrowAngle)
				);
				ctx.moveTo(x, y);
				ctx.lineTo(
					x - arrowLength * Math.cos(angle + arrowAngle),
					y - arrowLength * Math.sin(angle + arrowAngle)
				);
				ctx.stroke();
			};

			const drawDashedLine = (x1, y1, x2, y2) => {
				ctx.setLineDash([2, 4]);
				ctx.beginPath();
				ctx.moveTo(x1, y1);
				ctx.lineTo(x2, y2);
				ctx.stroke();
				ctx.setLineDash([]);
			};

			switch (dir) {
				case 1: // 北
					for (let i = 0; i < movs.length; i++) {
						const mov = movs.substr(i, 1);
						switch (mov) {
							case '直':
								ctx.beginPath();
								ctx.moveTo(w/4, h/10);
								ctx.lineTo(w/4, h*3/5);
								ctx.stroke();
								drawArrow(w/4, h*3/5, Math.PI/2);
								break;
							case '左':
								ctx.beginPath();
								ctx.moveTo(w/4, h/10);
								ctx.quadraticCurveTo(w/4, h/4, w*3/8, h*3/8);
								ctx.stroke();
								drawArrow(w*3/8, h*3/8, Math.PI/4);
								break;
							case '右':
								ctx.beginPath();
								ctx.moveTo(w/4, h/10);
								ctx.quadraticCurveTo(w/4, h/4, w/8, h*3/8);
								ctx.stroke();
								drawArrow(w/8, h*3/8, Math.PI/2 + Math.PI/6);
								break;
							case '掉':
								ctx.beginPath();
								ctx.moveTo(w/4, h/10);
								ctx.quadraticCurveTo(w*5/16, h*3/8, w*3/8, h/4);
								ctx.lineTo(w*3/8, h/10);
								ctx.stroke();
								drawArrow(w*3/8, h/10, 0);
								break;
							case '都':
								drawDashedLine(w/20, h/20, w*19/20, h/20);
								drawArrow(w/20, h/20, Math.PI);
								drawArrow(w*19/20, h/20, 0);
								break;
							case '进':
								drawDashedLine(w/20, h/20, w*7/16, h/20);
								drawArrow(w/20, h/20, Math.PI);
								drawArrow(w*7/16, h/20, 0);
								break;
							case '出':
								drawDashedLine(w*9/16, h/20, w*19/20, h/20);
								drawArrow(w*9/16, h/20, Math.PI);
								drawArrow(w*19/20, h/20, 0);
								break;
						}
					}
					break;
				case 3: // 东
					for (let i = 0; i < movs.length; i++) {
						const mov = movs.substr(i, 1);
						switch (mov) {
							case '直':
								ctx.beginPath();
								ctx.moveTo(w*9/10, h/4);
								ctx.lineTo(w*2/5, h/4);
								ctx.stroke();
								drawArrow(w*2/5, h/4, Math.PI);
								break;
							case '左':
								ctx.beginPath();
								ctx.moveTo(w*9/10, h/4);
								ctx.quadraticCurveTo(w*3/4, h/4, w*5/8, h*3/8);
								ctx.stroke();
								drawArrow(w*5/8, h*3/8, Math.PI- Math.PI/4);
								break;
							case '右':
								ctx.beginPath();
								ctx.moveTo(w*9/10, h/4);
								ctx.quadraticCurveTo(w*3/4, h/4, w*5/8, h/8);
								ctx.stroke();
								drawArrow(w*5/8, h/8, Math.PI+ Math.PI/4);
								break;
							case '掉':
								ctx.beginPath();
								ctx.moveTo(w*9/10, h/4);
								ctx.quadraticCurveTo(w*5/8, h*5/16, w*6/8, h*3/8);
								ctx.lineTo(w*9/10, h*3/8);
								ctx.stroke();
								drawArrow(w*9/10, h*3/8, Math.PI/2);
								break;
							case '都':
								drawDashedLine(w*19/20, h/20, w*19/20, h*19/20);
								drawArrow(w*19/20, h/20, -Math.PI/2);
								drawArrow(w*19/20, h*19/20, Math.PI/2);
								break;
							case '进':
								drawDashedLine(w*19/20, h/20, w*19/20, h*7/16);
								drawArrow(w*19/20, h/20, -Math.PI/2);
								drawArrow(w*19/20, h*7/16, Math.PI/2);
								break;
							case '出':
								drawDashedLine(w*19/20, h*9/16, w*19/20, h*19/20);
								drawArrow(w*19/20, h*9/16, -Math.PI/2);
								drawArrow(w*19/20, h*19/20, Math.PI/2);
								break;
						}
					}
					break;
				case 5: // 南
					for (let i = 0; i < movs.length; i++) {
						const mov = movs.substr(i, 1);
						switch (mov) {
							case '直':
								ctx.beginPath();
								ctx.moveTo(w*6/8, h*9/10);
								ctx.lineTo(w*6/8, h*2/5);
								ctx.stroke();
								drawArrow(w*6/8, h*2/5, -Math.PI/2);
								break;
							case '左':
								ctx.beginPath();
								ctx.moveTo(w*6/8, h*9/10);
								ctx.quadraticCurveTo(w*6/8, h*6/8, w*5/8, h*5/8);
								ctx.stroke();
								drawArrow(w*5/8, h*5/8, -3*Math.PI/4);
								break;
							case '右':
								ctx.beginPath();
								ctx.moveTo(w*6/8, h*9/10);
								ctx.quadraticCurveTo(w*6/8, h*6/8, w*7/8, h*5/8);
								ctx.stroke();
								drawArrow(w*7/8, h*5/8, -Math.PI/4);
								break;
							case '掉':
								ctx.beginPath();
								ctx.moveTo(w*6/8, h*9/10);
								ctx.quadraticCurveTo(w*11/16, h*5/8, w*5/8, h*6/8);
								ctx.lineTo(w*5/8, h*9/10);
								ctx.stroke();
								drawArrow(w*5/8, h*9/10, -Math.PI/2);
								break;
							case '都':
								drawDashedLine(w/20, h*19/20, w*19/20, h*19/20);
								drawArrow(w/20, h*19/20, Math.PI);
								drawArrow(w*19/20, h*19/20, Math.PI*2);
								break;
							case '进':
								drawDashedLine(w*9/16, h*19/20, w*19/20, h*19/20);
								drawArrow(w*9/16, h*19/20, 0);
								drawArrow(w*19/20, h*19/20, Math.PI);
								break;
							case '出':
								drawDashedLine(w/20, h*19/20, w*7/16, h*19/20);
								drawArrow(w/20, h*19/20, 0);
								drawArrow(w*7/16, h*19/20, Math.PI);
								break;
						}
					}
					break;
				case 7: // 西
					for (let i = 0; i < movs.length; i++) {
						const mov = movs.substr(i, 1);
						switch (mov) {
							case '直':
								ctx.beginPath();
								ctx.moveTo(w/10, h*6/8);
								ctx.lineTo(w*3/5, h*6/8);
								ctx.stroke();
								drawArrow(w*3/5, h*6/8, 2*Math.PI);
								break;
							case '左':
								ctx.beginPath();
								ctx.moveTo(w/10, h*6/8);
								ctx.quadraticCurveTo(w*2/8, h*6/8, w*3/8, h*5/8);
								ctx.stroke();
								drawArrow(w*3/8, h*5/8,  -Math.PI/4);
								break;
							case '右':
								ctx.beginPath();
								ctx.moveTo(w/10, h*6/8);
								ctx.quadraticCurveTo(w*2/8, h*6/8, w*3/8, h*7/8);
								ctx.stroke();
								drawArrow(w*3/8, h*7/8, Math.PI/4);
								break;
							case '掉':
								ctx.beginPath();
								ctx.moveTo(w/10, h*6/8);
								ctx.quadraticCurveTo(w*3/8, h*11/16, w*2/8, h*5/8);
								ctx.lineTo(w/10, h*5/8);
								ctx.stroke();
								drawArrow(w/10, h*5/8, -Math.PI/2);
								break;
							case '都':
								drawDashedLine(w/20, h/20, w/20, h*19/20);
								drawArrow(w/20, h/20, -Math.PI/2);
								drawArrow(w/20, h*19/20, Math.PI/2);
								break;
							case '进':
								drawDashedLine(w/20, h*9/16, w/20, h*19/20);
								drawArrow(w/20, h*9/16, -Math.PI/2);
								drawArrow(w/20, h*19/20, Math.PI/2);
								break;
							case '出':
								drawDashedLine(w/20, h/20, w/20, h*7/16);
								drawArrow(w/20, h/20, -Math.PI/2);
								drawArrow(w/20, h*7/16, Math.PI/2);
								break;
						}
					}
					break;
			}

			if (scale != 1) {
				ctx.scale(1/scale, 1/scale);
			}
		}
	}
}
/*
dir:
0	北
1	东北
2	东
3	东南
4	南
5	西南
6	西
7	西北
movement:
11	直行
12	左转
13	右转
21	直左混行
22	直右混行
23	左右混行
24	直左右混行
31	掉头
99	其他
灯组
10:'机动车信号灯',
11:'机动车直行箭头灯',
12:'机动车左转箭头灯',
13:'机动车右转箭头灯',
14:'机动车掉头箭头灯',
21:'非机动车灯',
22:'直行非机动车灯',
23:'左转非机动车灯',
31:'行人灯',
32:'行人灯-进口',
33:'行人灯-出口',
99:'其他',
*/
</script>

<style></style>
