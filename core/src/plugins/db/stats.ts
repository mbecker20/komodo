import { DiskStats, MemStats, StoredStats } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";

const stats = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const MemSchema = new Schema<MemStats>({
		totalMemMb: Number,
		usedMemPercentage: Number,
	});

	const DiskSchema = new Schema<DiskStats>({
		totalGb: Number,
		usedPercentage: Number,
	});

	const schema = new Schema<StoredStats>({
		serverID: { type: String, index: true },
		ts: { type: Number, index: true },
		cpu: Number,
		mem: MemSchema,
		disk: DiskSchema
	});
	
	app.decorate("stats", app.mongoose.model("Stats", schema));
	
	done();
});

export default stats;