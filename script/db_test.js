
db.Event.find({ kind: 1 }).sort({ created_at: -1 });


db.Event.findOne({
    id: "4376c65d2f232afbe9b882a35baa4f6fe8667c4e684749af565f981833ed6a65"
});


db.Event.deleteOne({
    id: "4376c65d2f232afbe9b882a35baa4f6fe8667c4e684749af565f981833ed6a65"
});