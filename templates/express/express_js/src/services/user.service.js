const User = require('../models/user.model');

exports.create = async (data) => {
  return await User.create(data);
};

exports.getAll = async () => {
  return await User.find();
};

exports.getById = async (id) => {
  return await User.findById(id);
};

exports.update = async (id, data) => {
  return await User.findByIdAndUpdate(id, data, {
    new: true,
  });
};

exports.remove = async (id) => {
  return await User.findByIdAndDelete(id);
};