exports.successResponse = (res, data, message = 'Success') => {
  return res.json({
    success: true,
    message,
    data,
  });
};

exports.errorResponse = (
  res,
  message = 'Something went wrong',
  status = 500
) => {
  return res.status(status).json({
    success: false,
    message,
  });
};