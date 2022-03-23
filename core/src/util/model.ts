import { Collection } from "@monitor/types";
import { intoCollection, objFrom2Arrays } from "@monitor/util";
import { FastifyInstance } from "fastify";
import {
  Schema,
  FilterQuery,
  QueryOptions,
  UpdateQuery,
  UpdateWithAggregationPipeline,
} from "mongoose";

// a custom api on top of mongoose models with assumptive types / custom convenience queries
const model = <T>(app: FastifyInstance, name: string, schema: Schema<T>) => {
  const model = app.mongoose.model<T>(name, schema);

  return {
    create: async (item: T) => {
      return (await model.create(item)).toObject() as T;
    },
    find: async (
      filter: FilterQuery<T> = {},
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model.find(filter, projection, options).lean().exec()) as T[];
    },
    findById: async (
      id: string,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model.findById(id, projection, options).lean().exec()) as
        | T
        | undefined;
    },
    findByField: async <Target>(
      field: string,
      expr: Target,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model
        .find({ [field]: expr } as FilterQuery<T>, projection, options)
        .lean()
        .exec()) as T[];
    },
    findOne: async (
      filter: FilterQuery<T> = {},
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model
        .findOne(filter, projection, options)
        .lean()
        .exec()) as T | undefined;
    },
    getMostRecent: async (
      limit: number,
      filter: FilterQuery<T>,
      offset = 0,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model
        .find(filter, projection, options)
        .sort({ createdAt: -1 })
        .skip(offset)
        .limit(limit)
        .lean()
        .exec()) as T[];
    },
    findCollection: async (
      filter: FilterQuery<T>,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      const docs = await model.find(filter, projection, options).lean().exec();
      return intoCollection(
        docs as T[],
      ) as Collection<T>;
    },
    findByIdAndDelete: async (id: string) => {
      return (await model.findByIdAndDelete(id).lean().exec()) as T | undefined;
    },
    updateMany: async (
      filter: FilterQuery<T>,
      update: UpdateQuery<T> | UpdateWithAggregationPipeline,
      options?: QueryOptions
    ) => {
      return await model.updateMany(filter, update, options).lean().exec();
    },
    updateOne: async (
      filter: FilterQuery<T>,
      update: UpdateQuery<T> | UpdateWithAggregationPipeline,
      options?: QueryOptions
    ) => {
      return await model.updateOne(filter, update, options).lean().exec();
    },
    updateById: async (
      _id: string,
      update: UpdateQuery<T> | UpdateWithAggregationPipeline,
      options?: QueryOptions
    ) => {
      return await model.updateOne({ _id }, update, options).lean().exec();
    },
  };
};

// this is just used to derive the returntype of a the generic model
class CreateModel<T> {
  mediate = (app: FastifyInstance, name: string, schema: Schema<T>) =>
    model<T>(app, name, schema);
}

export type Model<T> = ReturnType<CreateModel<T>["mediate"]>;

export default model;
